"""Render side-by-side mesh comparisons for the best and the worst cases.

Each image shows the same part twice:

  left   the reference mesh, solid green -- the OCCT mesher run at a fixed
         chord tolerance of 1e-3 (see ``harness.align --generate-refs``)
  right  the kernel mesh, coloured per vertex by its distance to the
         reference (green = sits on the reference surface, red = far off)

Both meshes come from the *same* OCCT kernel, meshed at different chord
tolerances, so these pictures show tessellation sensitivity, not a
cross-kernel comparison.

Selection: every failing case, plus the ``N_BEST`` best-matching passing
cases.  Output goes to ``docs/visualizations/mesh_renders/``.
"""

from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[2]
FIGDIR = ROOT / "docs" / "visualizations"
MESHDIR = FIGDIR / "mesh_renders"
RESULTS = ROOT / "harness" / "viz" / "alignment_results.json"

REF_DIR = ROOT / "results" / "refs"
MESH_DIR = ROOT / "results" / "meshes" / "occt"

# The thresholds the harness grades against.  The derived score below is the
# mean of the three metrics, each expressed as a fraction of its threshold.
TOL_HAUSDORFF = 0.05
TOL_VERTEX_RMSE = 0.02
TOL_NORMAL_DEG = 20.0

# How many of the best-matching passing cases to show next to the failures.
N_BEST = 6

import matplotlib  # noqa: F401  (reuse the same matplotlibrc as the charts)
import numpy as np
import pyvista as pv
import trimesh

pv.OFF_SCREEN = True


def derived_score(r):
    """Mean of the three metrics, each as a fraction of its threshold."""
    return (
        r["hausdorff_rel"] / TOL_HAUSDORFF
        + r["vertex_rmse_rel"] / TOL_VERTEX_RMSE
        + r["normal_deviation_deg"] / TOL_NORMAL_DEG
    ) / 3


def pick_cases(results):
    """The failing cases, then the N_BEST best-matching passing cases."""
    for r in results:
        r["derived_score"] = derived_score(r)
    failing = sorted(
        (r for r in results if not r["passed"]), key=lambda r: r["derived_score"]
    )
    passing = sorted(
        (r for r in results if r["passed"]), key=lambda r: r["derived_score"]
    )
    return failing + passing[:N_BEST]


def render_pair(case, out_path, kernel_name="OCCT"):
    """Reference (green) and kernel mesh (coloured by distance) side by side."""
    ref_stl = REF_DIR.joinpath(*case.split("/")).with_suffix(".stl")
    kernel_stl = MESH_DIR.joinpath(*case.split("/")).with_suffix(".stl")
    if not ref_stl.exists() or not kernel_stl.exists():
        print(f"  skipped (missing mesh): {case}")
        return False

    ref_mesh = trimesh.load(str(ref_stl), force="mesh")
    kernel_mesh = trimesh.load(str(kernel_stl), force="mesh")
    _, distances, _ = trimesh.proximity.closest_point(ref_mesh, kernel_mesh.vertices)
    max_dist = float(np.max(distances)) or 1.0

    plotter = pv.Plotter(off_screen=True, window_size=[1600, 800], shape=(1, 2))

    # Left: the reference mesh, uncoloured.
    plotter.subplot(0, 0)
    plotter.add_mesh(
        pv.wrap(ref_mesh),
        color="green",
        show_edges=True,
        edge_color="darkgreen",
        opacity=0.9,
    )
    plotter.add_text(f"Reference (Control)\n{case}", font_size=14)
    plotter.view_isometric()
    plotter.camera.zoom(1.2)

    # Right: the kernel mesh, coloured by distance to the reference.
    plotter.subplot(0, 1)
    kernel_pv = pv.wrap(kernel_mesh)
    kernel_pv.point_data["distance"] = distances
    plotter.add_mesh(
        kernel_pv,
        scalars="distance",
        cmap="RdYlGn_r",
        show_edges=True,
        edge_color="gray",
        opacity=0.9,
        scalar_bar_args={
            "title": "Distance to Reference",
            "vertical": True,
            "n_labels": 5,
        },
    )
    plotter.add_text(
        f"{kernel_name} Kernel Mesh\n{case}\nMax distance: {max_dist:.4f}",
        font_size=14,
    )
    plotter.view_isometric()
    plotter.camera.zoom(1.2)

    plotter.link_views()
    out_path.parent.mkdir(parents=True, exist_ok=True)
    plotter.screenshot(str(out_path))
    plotter.close()
    print(f"  wrote {out_path.relative_to(ROOT)}")
    return True


def main():
    with open(RESULTS) as f:
        results = json.load(f)

    cases = pick_cases(results)
    failing = sum(1 for r in cases if not r["passed"])
    best = sum(1 for r in cases if r["passed"])
    print(f"Rendering {len(cases)} cases ({failing} failing, {best} best passing)")

    for r in cases:
        # The results file stores case names with a Windows-style separator.
        case = r["case"].replace("\\", "/")
        out_path = MESHDIR / f"mesh_{case.replace('/', '_')}.png"
        render_pair(case, out_path)


if __name__ == "__main__":
    main()
