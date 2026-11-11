# Visualization scripts

These scripts turn the raw `(reference, kernel)` mesh pairs under `results/`
into the charts and mesh renders in [`docs/visualizations/`](../../docs/visualizations/).
They never modify the meshes, the harness, or the test cases.

Run any of them from anywhere inside the repo:

```sh
python harness/viz/collect_results.py
```

## What each one does

| Script | Produces |
| --- | --- |
| `collect_results.py` | Grades every mesh pair with `harness.align.grade_alignment` and writes `harness/viz/alignment_results.json` (the input for every other script). |
| `create_viz.py` | Bar charts, scatter plots, per-category summaries, and `alignment_summary.txt`. |
| `create_derived_viz.py` | The "derived-ness" gradient / best-vs-worst / per-category charts. |
| `render_meshes.py` | 3D renders of every failing case plus the six best-matching passing cases (reference in green, kernel coloured by per-vertex distance) into `docs/visualizations/mesh_renders/`. |

## Order

Run `collect_results.py` first; every other script reads its
`alignment_results.json`. The other three are independent of each other.

## Requirements

`numpy`, `matplotlib`, `trimesh`, `pyvista`. Rendering is headless
(`pyvista.OFF_SCREEN = True`).

The renders need the mesh pairs to exist on disk: generate the reference
meshes with `python -m harness.align --generate-refs` and the kernel meshes by
running the harness (`python -m harness.align --kernel occt --all`).
