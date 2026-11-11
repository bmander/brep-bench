#!/usr/bin/env python3
"""The brep-bench runner for SMLib, through NVIDIA's `usd-brep` Python package.

    python3 kernels/smlib/run.py CASE OUT.stl     # CASE is an id, e.g. boolean/napkin-ring
    python3 kernels/smlib/run.py --version

Loads the case's implementation, `cases/<category>/<name_snake>.py` beside this file, calls its
`build()`, and measures the solid it returns with SMLib's own tools: its volume and area
(SmApiBrepComputeVolume, SmApiBrepComputeArea), its faces, and whether it is a closed manifold
solid. It meshes the solid with SMLib's tessellator, writes the triangles to OUT.stl, and prints
one line of JSON (SPEC.md, "A runner"). Given `-` for OUT.stl (the harness's timed run), it builds
the solid, and measures and meshes nothing: SMLib has no STEP writer here. The runner never reads
the case's description: the implementation is the whole of the case here. Needs `usd-brep`.
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))

if len(sys.argv) > 1 and sys.argv[1] == "--version":
    from importlib.metadata import version
    print(f"SMLib via usd-brep {version('usd-brep')}")
    sys.exit(0)

sys.path.insert(0, HERE)
from bench import Refused  # noqa: E402

# How finely the measures integrate. SMLib's documentation advises against asking for better than
# 1e-8, and at 1e-8 the integration of some solids does not converge and the call fails. The
# runner asks for 1e-8 and, only where that fails, for the next coarser accuracy, down to the
# benchmark's own 1e-6.
ACCURACIES = (1e-8, 1e-7, 1e-6)

# How near the mesh keeps to the solid: a fraction of the diagonal of the solid's bounding box,
# and no more than a fraction of its thickness (volume over area), as the reference runner does.
MESH_DEFLECTION = 1e-4
THICKNESS_DEFLECTION = 5e-3


def implementation(case_id):
    """The module implementing a case: cases/<category>/<name_snake>.py."""
    category, name = case_id.split("/")
    path = os.path.join(HERE, "cases", category, name.replace("-", "_") + ".py")
    if not os.path.exists(path):
        sys.exit(f"no implementation of {case_id}")
    spec = importlib.util.spec_from_file_location(f"cases.{category}.{name}", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def integrate(measure):
    """A measure at the finest accuracy its integration converges at."""
    for accuracy in ACCURACIES:
        try:
            return measure(accuracy)
        except RuntimeError:
            if accuracy == ACCURACIES[-1]:
                raise


def face_kinds(solid):
    """The solid's faces counted by the kind of their surface. The package tells a plane from
    any other surface and no more, and SMLib holds its curved surfaces as NURBS."""
    kinds = {}
    for face in solid.faces():
        kind = "plane" if face.surface().is_planar() else "bspline"
        kinds[kind] = kinds.get(kind, 0) + 1
    return kinds


def write_stl(mesh, out_stl):
    """The mesh's triangles as an ASCII STL, its coordinates in full: a binary STL's single
    precision would merge points a micron apart. A vertex shared by two triangles is written
    from the one point the tessellator gave it, so nothing is welded here."""
    points, stream = (mesh.get_points(), mesh.to_mesh_arrays()["faces"]) if mesh else ([], [])
    with open(out_stl, "w") as f:
        f.write("solid smlib\n")
        i = 0
        while i < len(stream):
            n = stream[i]
            corners = [points[k] for k in stream[i + 1:i + 1 + n]]
            for j in range(1, n - 1):
                f.write("facet normal 0 0 0\nouter loop\n")
                for x, y, z in (corners[0], corners[j], corners[j + 1]):
                    f.write(f"vertex {x!r} {y!r} {z!r}\n")
                f.write("endloop\nendfacet\n")
            i += 1 + n
        f.write("endsolid smlib\n")


def measure(solid, out_stl):
    if solid.face_count() == 0:
        # an operation returned no solid at all, without declining: nothing to mesh
        write_stl(None, out_stl)
        return {"status": "ok", "volume": 0.0, "area": 0.0, "faces": {}, "valid": False}
    volume = integrate(solid.volume)
    area = integrate(solid.area)
    reply = {"status": "ok", "volume": volume, "area": area,
             "faces": face_kinds(solid), "valid": bool(solid.is_manifold_solid())}

    low, high = solid.bounding_box()
    diagonal = sum((h - l) ** 2 for l, h in zip(low, high)) ** 0.5
    thickness = volume / area if area > 0 else diagonal
    deflection = min(MESH_DEFLECTION * diagonal, THICKNESS_DEFLECTION * thickness)
    write_stl(solid.tessellate(chord_height_tolerance=deflection), out_stl)
    return reply


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: run.py CASE OUT.stl")
    case_id, out_stl = sys.argv[1], sys.argv[2]
    build = implementation(case_id).build
    try:
        solid = build()
        reply = {"status": "ok"} if out_stl == "-" else measure(solid, out_stl)
    except Refused as why:
        reply = {"status": "refused", "reason": str(why)}
    print(json.dumps(reply))


if __name__ == "__main__":
    main()
