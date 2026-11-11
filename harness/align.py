#!/usr/bin/env python3
"""Mesh alignment & topology grading for brep-bench.

Addresses Issue #18 Section 1: centroid, inertia, topology (genus/lumps/shells),
non-manifold support, empty results, placement invariance.

    python3 -m harness.align [--kernel KERNEL] [--case CASE] [--all] [--generate-refs]
                              [--chord-tol 1e-3] [--tol-vertex 1e-3] [--tol-hausdorff 1e-2]

Outputs JSON per case compatible with the board schema, with an 'align' object:
{
  "case": "boolean/napkin-ring",
  "kernel": "occt",
  "align": {
    "centroid_error": 1.2e-6,
    "inertia_axes_aligned": true,
    "hausdorff": 3.4e-4,
    "vertex_rmse": 2.1e-4,
    "normal_deviation_deg": 0.15,
    "topology": {"lumps": 1, "genus": [0], "shells": 1, "euler": 2},
    "passed": true
  }
}
"""
import argparse
import json
import os
import sys
from pathlib import Path

try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False
    np = None

try:
    from scipy.spatial import cKDTree
    from scipy.spatial.distance import cdist
    HAS_SCIPY = True
except ImportError:
    HAS_SCIPY = False
    cKDTree = None
    cdist = None

# Make sure we can import local harness modules
HERE = Path(__file__).parent
ROOT = HERE.parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(HERE))

from harness import suite  # noqa: E402
import harness.stl as stl  # noqa: E402

# Try to import trimesh for re-meshing
try:
    import trimesh
    HAS_TRIMESH = True
except ImportError:
    HAS_TRIMESH = False
    trimesh = None

assert np is not None, "numpy required for mesh alignment"


def remesh_to_target(V, F, target_edge_len):
    """Re-mesh a triangle mesh to a target resolution using trimesh.

    Returns (V_new, F_new) or (V, F) if trimesh unavailable or re-mesh fails.

    Uses quadratic decimation to a fixed target vertex count for consistency.
    Subdivision is avoided as it is unreliable for complex meshes.
    """
    if not HAS_TRIMESH or trimesh is None or not HAS_NUMPY or np is None:
        return V, F

    try:
        mesh = trimesh.Trimesh(vertices=V, faces=F, process=False)
        if len(mesh.vertices) == 0 or len(mesh.faces) == 0:
            return V, F

        # Fixed target vertex count for consistent comparison
        # This avoids the unreliable edge-length-based subdivision
        target_vertices = 5000

        # Clamp to reasonable range
        target_vertices = max(500, min(target_vertices, 50000))

        current_vertices = len(mesh.vertices)
        # If already close to target, don't re-mesh
        if current_vertices > target_vertices * 0.8 and current_vertices < target_vertices * 1.2:
            return mesh.vertices, mesh.faces

        # Use quadratic decimation to reach target vertex count (fast and stable)
        if current_vertices > target_vertices:
            target_faces = int(target_vertices * 1.5)  # V ~ 2F/3 => F ~ 1.5V
            mesh = mesh.simplify_quadric_decimation(face_count=target_faces)
            return mesh.vertices, mesh.faces

        # Current mesh is already finer than target - don't subdivide (unreliable)
        # Just return as-is
        return mesh.vertices, mesh.faces
    except Exception:
        return V, F


def canonical_frame(vertices, faces):
    """Return (V_canon, centroid, R) where V_canon = (V - c) @ R.T.

    Frame: origin = centroid (volume-weighted), axes = principal inertia axes
    (sorted by eigenvalue ascending), right-handed (det(R) > 0), deterministic
    sign via max-component rule.
    """
    if np is None:
        raise RuntimeError("numpy required for canonical_frame")
    # Build trimesh-like object for inertia
    # Volume-weighted centroid of a closed mesh:
    # c = sum_i V_i * (v_i0 + v_i1 + v_i2) / (4 * sum_i V_i)
    # where V_i is signed volume of tetrahedron (0, v0, v1, v2)
    V = vertices[faces]
    v0, v1, v2 = V[:, 0], V[:, 1], V[:, 2]

    # Cross products for face normals (2 * area * normal)
    u = v1 - v0
    w = v2 - v0
    cross = np.cross(u, w)  # shape (n_faces, 3)

    # Signed volume of each tetrahedron (0, v0, v1, v2)
    vols = np.sum(v0 * cross, axis=1) / 6.0
    total_vol = np.sum(vols)

    if abs(total_vol) < 1e-15:
        # Degenerate: fall back to vertex mean
        c = vertices.mean(axis=0)
        # Need centroids for inertia even in degenerate case
        centroids = (v0 + v1 + v2) / 3.0
    else:
        # Volume-weighted centroid
        centroids = (v0 + v1 + v2) / 3.0
        c = np.sum(centroids.T * vols, axis=1) / total_vol

    # Inertia tensor about centroid (unit density)
    # I_xx = sum V_i * (y^2 + z^2) for each tetrahedron's centroid
    # Using parallel axis theorem on face centroids is approximate but fast
    # For exact: integrate over tetrahedra. Approximation is fine for grading.
    r = centroids - c
    r2 = np.sum(r * r, axis=1)
    I = np.zeros((3, 3))
    for i in range(3):
        for j in range(3):
            if i == j:
                I[i, i] = np.sum(vols * (r2 - r[:, i] ** 2))
            else:
                I[i, j] = -np.sum(vols * r[:, i] * r[:, j])

    # Eigendecomposition: I = R @ diag(eigvals) @ R.T
    eigvals, eigvecs = np.linalg.eigh(I)  # ascending eigenvalues
    R = eigvecs.T  # rows = principal axes

    # Enforce right-handed frame
    if np.linalg.det(R) < 0:
        R[0] *= -1

    # Deterministic sign: for each axis, flip so max abs component is positive
    for i in range(3):
        max_idx = np.argmax(np.abs(R[i]))
        if R[i, max_idx] < 0:
            R[i] *= -1

    V_canon = (vertices - c) @ R.T
    return V_canon, c, R


def load_stl(path):
    """Load STL, return (vertices, faces) as numpy arrays."""
    tris = stl.read(path)
    if not tris:
        return np.empty((0, 3)), np.empty((0, 3), dtype=int)
    # Build vertex index (unique vertices)
    verts = {}
    faces = []
    for a, b, c in tris:
        for v in (a, b, c):
            if v not in verts:
                verts[v] = len(verts)
        faces.append([verts[a], verts[b], verts[c]])
    V = np.array(list(verts.keys()), dtype=np.float64)
    F = np.array(faces, dtype=np.int32)
    return V, F


def mesh_topology(V, F):
    """Return (lumps, genus_per_lump, shells, euler_per_lump).

    Uses trimesh's split(only_watertight=False) logic via connected components
    of the vertex-face adjacency graph.
    """
    if len(F) == 0:
        return 0, [], 0, []

    # Build adjacency: face -> neighbor faces via shared edges
    edge_to_faces = {}
    for fi, (a, b, c) in enumerate(F):
        for e in ((a, b), (b, c), (c, a)):
            key = tuple(sorted(e))
            edge_to_faces.setdefault(key, []).append(fi)

    # Connected components of faces (lumps)
    visited = np.zeros(len(F), dtype=bool)
    lumps = []
    for i in range(len(F)):
        if not visited[i]:
            stack = [i]
            comp = []
            while stack:
                fi = stack.pop()
                if visited[fi]:
                    continue
                visited[fi] = True
                comp.append(fi)
                a, b, c = F[fi]
                for e in ((a, b), (b, c), (c, a)):
                    key = tuple(sorted(e))
                    for nfi in edge_to_faces.get(key, []):
                        if not visited[nfi]:
                            stack.append(nfi)
            lumps.append(comp)

    genus_list = []
    euler_list = []
    shells = 0
    for comp in lumps:
        # Euler characteristic: V - E + F
        # Count unique vertices and edges in this component
        comp_faces = F[comp]
        verts = set(comp_faces.ravel())
        edges = set()
        for a, b, c in comp_faces:
            for e in ((a, b), (b, c), (c, a)):
                edges.add(tuple(sorted(e)))
        Vc = len(verts)
        Ec = len(edges)
        Fc = len(comp_faces)
        euler = Vc - Ec + Fc
        euler_list.append(euler)
        # Genus for closed orientable: g = 1 - euler/2
        # For non-closed, this is still a useful invariant
        genus = 1 - euler / 2
        genus_list.append(round(genus, 6))
        # Shell = connected component of boundary (always 1 per lump for closed)
        shells += 1

    return len(lumps), genus_list, shells, euler_list


def sample_surface(V, F, n=10000):
    """Sample n points uniformly on mesh surface."""
    if np is None:
        raise RuntimeError("numpy required for sample_surface")
    if len(F) == 0:
        return np.empty((0, 3)), np.empty((0, 3))
    # Face areas
    v0, v1, v2 = V[F[:, 0]], V[F[:, 1]], V[F[:, 2]]
    cross = np.cross(v1 - v0, v2 - v0)
    areas = 0.5 * np.linalg.norm(cross, axis=1)
    if areas.sum() == 0:
        return np.empty((0, 3)), np.empty((0, 3))
    probs = areas / areas.sum()
    # Sample faces
    face_idx = np.random.choice(len(F), size=n, p=probs)
    # Barycentric coords
    r1 = np.random.random(n)
    r2 = np.random.random(n)
    # Ensure uniform on triangle: sqrt(r1) * (1-r2) for u, sqrt(r1)*r2 for v
    u = 1 - np.sqrt(r1)
    v = np.sqrt(r1) * (1 - r2)
    w = np.sqrt(r1) * r2
    f0, f1, f2 = V[F[face_idx, 0]], V[F[face_idx, 1]], V[F[face_idx, 2]]
    pts = u[:, None] * f0 + v[:, None] * f1 + w[:, None] * f2
    # Face normals
    normals = cross[face_idx]
    norms = np.linalg.norm(normals, axis=1, keepdims=True)
    normals = normals / np.maximum(norms, 1e-12)
    return pts, normals


def hausdorff_distance(V1, F1, V2, F2, sample_n=20000):
    """Approximate Hausdorff distance via dense sampling (bidirectional)."""
    if np is None:
        raise RuntimeError("numpy required for hausdorff_distance")
    pts1, _ = sample_surface(V1, F1, sample_n)
    pts2, _ = sample_surface(V2, F2, sample_n)
    if len(pts1) == 0 or len(pts2) == 0:
        return float('inf')
    # Use scipy's cKDTree if available, else brute force (slow)
    if HAS_SCIPY:
        assert cKDTree is not None
        tree1 = cKDTree(pts1)
        tree2 = cKDTree(pts2)
        d1, _ = tree1.query(pts2, k=1)
        d2, _ = tree2.query(pts1, k=1)
        return max(d1.max(), d2.max())
    # Brute force fallback (slow, for small meshes only)
    assert cdist is not None
    D = cdist(pts1, pts2)
    return max(D.min(axis=0).max(), D.min(axis=1).max())


def vertex_rmse(V1, F1, V2, F2, sample_n=10000):
    """Bidirectional vertex RMSE: sample points on each, find nearest on other."""
    if np is None:
        raise RuntimeError("numpy required for vertex_rmse")
    pts1, _ = sample_surface(V1, F1, sample_n)
    pts2, _ = sample_surface(V2, F2, sample_n)
    if len(pts1) == 0 or len(pts2) == 0:
        return float('inf')
    if HAS_SCIPY:
        assert cKDTree is not None
        tree1 = cKDTree(pts1)
        tree2 = cKDTree(pts2)
        d1, _ = tree1.query(pts2, k=1)
        d2, _ = tree2.query(pts1, k=1)
        return float(np.sqrt((np.mean(d1**2) + np.mean(d2**2)) / 2))
    assert cdist is not None
    D = cdist(pts1, pts2)
    d1 = D.min(axis=0)
    d2 = D.min(axis=1)
    return float(np.sqrt((np.mean(d1**2) + np.mean(d2**2)) / 2))


def normal_deviation(V1, F1, V2, F2, sample_n=5000):
    """Mean angular deviation between nearest face normals (degrees)."""
    if np is None:
        raise RuntimeError("numpy required for normal_deviation")
    pts1, n1 = sample_surface(V1, F1, sample_n)
    pts2, n2 = sample_surface(V2, F2, sample_n)
    if len(pts1) == 0 or len(pts2) == 0:
        return float('inf')
    if HAS_SCIPY:
        assert cKDTree is not None
        tree2 = cKDTree(pts2)
        _, idx = tree2.query(pts1, k=1)
        dots = np.clip(np.abs(np.sum(n1 * n2[idx], axis=1)), 0, 1)
        angles = np.arccos(dots)
        return float(np.degrees(angles.mean()))
    return float('inf')


def bbox_diagonal(V):
    if len(V) == 0:
        return 1.0
    return np.linalg.norm(V.max(axis=0) - V.min(axis=0))


def grade_alignment(kernel_mesh_path, ref_mesh_path, tol_vertex=0.02, tol_hausdorff=0.05, tol_normal=10.0, target_edge_len=1e-3):
    """Grade one kernel mesh against reference mesh.

    Returns dict with metrics and pass/fail.
    """
    Vk, Fk = load_stl(kernel_mesh_path)
    Vr, Fr = load_stl(ref_mesh_path)

    if len(Vk) == 0 or len(Fk) == 0:
        return {"error": "empty kernel mesh", "passed": False}

    if len(Vr) == 0 or len(Fr) == 0:
        return {"error": "empty reference mesh", "passed": False}

    # Re-meshing disabled for now - compare at native resolution
    # Vk, Fk = remesh_to_target(Vk, Fk, target_edge_len)
    # Vr, Fr = remesh_to_target(Vr, Fr, target_edge_len)

    if len(Vk) == 0 or len(Fk) == 0 or len(Vr) == 0 or len(Fr) == 0:
        return {"error": "empty mesh", "passed": False}

    # Compute target edge length relative to model size if target is very small
    # This handles the case where chord_tol (1e-3) is an absolute deflection, not edge length
    diag_k = np.linalg.norm(Vk.max(axis=0) - Vk.min(axis=0))
    diag_r = np.linalg.norm(Vr.max(axis=0) - Vr.min(axis=0))
    diag = max(diag_k, diag_r)

    # If target_edge_len is unreasonably small for this model, derive from diagonal
    # Typical edge length for deflection d on radius R: sqrt(R*d). For box, use diag * 0.001
    # But also ensure target is not smaller than what the reference mesh achieves
    if target_edge_len < diag * 1e-4:
        target_edge_len = diag * 0.001  # ~1% of model diagonal

    # Clamp target to a reasonable range: not finer than 0.005 * diag, not coarser than 0.05 * diag
    target_edge_len = max(target_edge_len, diag * 0.005)
    target_edge_len = min(target_edge_len, diag * 0.05)

    # Re-mesh both to same target edge length to eliminate tessellation variance
    Vk, Fk = remesh_to_target(Vk, Fk, target_edge_len)
    Vr, Fr = remesh_to_target(Vr, Fr, target_edge_len)

    if len(Vk) == 0 or len(Fk) == 0 or len(Vr) == 0 or len(Fr) == 0:
        return {"error": "empty mesh after re-mesh", "passed": False}

    # Canonical frames
    Vk_c, ck, Rk = canonical_frame(Vk, Fk)
    Vr_c, cr, Rr = canonical_frame(Vr, Fr)

    # Centroid error in canonical frame (should be ~0 after alignment)
    centroid_error = float(np.linalg.norm(ck - cr))

    # Inertia axes alignment: check only axes with unique eigenvalues
    # Recompute eigenvalues to detect symmetry
    def get_inertia_eigvals(V, F):
        import trimesh
        mesh = trimesh.Trimesh(vertices=V, faces=F, process=False)
        return np.linalg.eigvalsh(mesh.moment_inertia)

    eigvals_k = get_inertia_eigvals(Vk, Fk)
    eigvals_r = get_inertia_eigvals(Vr, Fr)

    # Check alignment only for axes with distinct eigenvalues (tolerance 5% relative)
    def is_unique_axis(eigvals, i):
        for j in range(3):
            if i != j and abs(eigvals[i] - eigvals[j]) / max(abs(eigvals[i]), 1e-12) < 0.05:
                return False
        return True

    axes_aligned = True
    for i in range(3):
        if is_unique_axis(eigvals_k, i) and is_unique_axis(eigvals_r, i):
            if abs(np.dot(Rk[i], Rr[i])) <= 0.9999:
                axes_aligned = False
                break

    # Bbox diagonal for relative thresholds
    diag = max(bbox_diagonal(Vk), bbox_diagonal(Vr))
    if diag == 0:
        diag = 1.0

    # Shape metrics
    hausdorff = hausdorff_distance(Vk_c, Fk, Vr_c, Fr)
    v_rmse = vertex_rmse(Vk_c, Fk, Vr_c, Fr)
    normal_dev = normal_deviation(Vk_c, Fk, Vr_c, Fr)

    # Relative metrics
    hausdorff_rel = hausdorff / diag
    v_rmse_rel = v_rmse / diag

    # Topology
    lumps_k, genus_k, shells_k, euler_k = mesh_topology(Vk, Fk)
    lumps_r, genus_r, shells_r, euler_r = mesh_topology(Vr, Fr)

    topo_pass = (
        lumps_k == lumps_r
        and shells_k == shells_r
        and all(abs(gk - gr) < 0.5 for gk, gr in zip(genus_k, genus_r))
    )

    shape_pass = (
        hausdorff_rel <= tol_hausdorff
        and v_rmse_rel <= tol_vertex
        and normal_dev <= tol_normal  # degrees
    )

    passed = topo_pass and shape_pass

    return {
        "centroid_error": centroid_error,
        "centroid_error_rel": centroid_error / diag,
        "inertia_axes_aligned": axes_aligned,
        "hausdorff": float(hausdorff),
        "hausdorff_rel": float(hausdorff / diag),
        "vertex_rmse": float(v_rmse),
        "vertex_rmse_rel": float(v_rmse / diag),
        "normal_deviation_deg": float(normal_dev),
        "topology": {
            "lumps": lumps_k,
            "genus": genus_k,
            "shells": shells_k,
            "euler": euler_k,
        },
        "ref_topology": {
            "lumps": lumps_r,
            "genus": genus_r,
            "shells": shells_r,
            "euler": euler_r,
        },
        "thresholds": {
            "tol_vertex_rel": tol_vertex,
            "tol_hausdorff_rel": tol_hausdorff,
            "tol_normal_deg": tol_normal,
        },
        "passed": passed,
    }


def generate_reference_meshes(chord_tol=1e-3):
    """Run OCCT runner for all cases to generate reference meshes at fixed chord tolerance.

    This patches the OCCT runner's MESH_DEFLECTION to use the fixed value.
    """
    import subprocess
    cases_list = suite.cases()
    ref_dir = ROOT / "results" / "refs"
    ref_dir.mkdir(parents=True, exist_ok=True)

    occt_runner = ROOT / "kernels" / "occt" / "run.py"
    if not occt_runner.exists():
        print("OCCT runner not found", file=sys.stderr)
        return False

    # Use the same Python interpreter that's running this script
    python_exe = sys.executable

    for c in cases_list:
        out_stl = ref_dir / (c["id"] + ".stl")
        if out_stl.exists():
            print(f"  skip {c['id']} (exists)")
            continue
        print(f"  generating {c['id']}...")
        # Ensure parent directory exists
        out_stl.parent.mkdir(parents=True, exist_ok=True)
        # We need to override MESH_DEFLECTION in the runner
        # Easiest: run with env var and patch the runner on the fly
        env = os.environ.copy()
        env["BREP_BENCH_MESH_DEFLECTION"] = str(chord_tol)
        result = subprocess.run(
            [python_exe, str(occt_runner), c["id"], str(out_stl)],
            capture_output=True,
            text=True,
            cwd=ROOT,
            env=env,
            timeout=120,
        )
        if result.returncode != 0:
            print(f"    FAILED (code {result.returncode}):")
            print(f"    stdout: {result.stdout[:500]}")
            print(f"    stderr: {result.stderr[:1000]}", file=sys.stderr)
        else:
            print(f"    ok")

    print(f"Reference meshes in {ref_dir}")
    return True


def main():
    ap = argparse.ArgumentParser(description="Mesh alignment & topology grading")
    ap.add_argument("--kernel", action="append", help="Kernel name (repeatable)")
    ap.add_argument("--case", default="", help="Case id prefix")
    ap.add_argument("--all", action="store_true", help="Grade all kernels and cases")
    ap.add_argument("--generate-refs", action="store_true", help="Generate OCCT reference meshes")
    ap.add_argument("--chord-tol", type=float, default=1e-3, help="Chord tolerance for ref mesh generation")
    ap.add_argument("--tol-vertex", type=float, default=0.02, help="Relative vertex RMSE tolerance (fraction of diagonal)")
    ap.add_argument("--tol-hausdorff", type=float, default=0.05, help="Relative Hausdorff tolerance (fraction of diagonal)")
    ap.add_argument("--tol-normal", type=float, default=20.0, help="Normal deviation tolerance (degrees)")
    ap.add_argument("--output", help="Write results to JSON file")
    args = ap.parse_args()

    if args.generate_refs:
        ok = generate_reference_meshes(args.chord_tol)
        sys.exit(0 if ok else 1)

    # Determine which kernels and cases
    all_kernels = [k for k in suite.kernels() if k.get("command")]
    if args.kernel:
        kernels = [k for k in all_kernels if k["name"] in args.kernel]
    else:
        kernels = all_kernels

    chosen_cases = suite.cases(args.case) if args.case else suite.cases()

    ref_dir = ROOT / "results" / "refs"
    if not ref_dir.exists():
        print("Reference meshes not found. Run with --generate-refs first.", file=sys.stderr)
        sys.exit(1)

    results = []
    for k in kernels:
        mesh_dir = ROOT / "results" / "meshes" / k["name"]
        if not mesh_dir.exists():
            print(f"No meshes for kernel {k['name']}", file=sys.stderr)
            continue
        for c in chosen_cases:
            kernel_mesh = mesh_dir / (c["id"] + ".stl")
            ref_mesh = ref_dir / (c["id"] + ".stl")
            if not kernel_mesh.exists():
                continue
            if not ref_mesh.exists():
                print(f"  no ref for {c['id']}", file=sys.stderr)
                continue
            print(f"  grading {k['name']} / {c['id']}...")
            res = grade_alignment(
                str(kernel_mesh), str(ref_mesh),
                tol_vertex=args.tol_vertex,
                tol_hausdorff=args.tol_hausdorff,
                tol_normal=args.tol_normal,
                target_edge_len=args.chord_tol,
            )
            res.update({"case": c["id"], "kernel": k["name"]})
            results.append(res)
            status = "PASS" if res.get("passed") else "FAIL"
            print(f"    {status}  hausdorff={res.get('hausdorff_rel', '?'):.3g}  "
                  f"vertex_rmse={res.get('vertex_rmse_rel', '?'):.3g}  "
                  f"lumps={res.get('topology', {}).get('lumps', '?')}")

    if args.output:
        with open(args.output, "w") as f:
            json.dump(results, f, indent=1)
            f.write("\n")

    # Print summary
    total = len(results)
    passed = sum(1 for r in results if r.get("passed"))
    print(f"\n{passed}/{total} passed")
    if total > 0 and passed < total:
        sys.exit(1)


if __name__ == "__main__":
    main()