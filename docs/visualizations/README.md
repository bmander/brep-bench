# Visualizations

Charts and mesh renders for the kernel-alignment run. Regenerate everything with
the scripts in [`harness/viz/`](../../harness/viz/) (see that README for details).

## What the renders show

Each render is a side-by-side pair for one test case:

- **Left — Reference**: the control mesh, generated locally by the harness with
  the OCCT mesher at a fixed coarse chord tolerance (`1e-3`).
- **Right — Kernel**: the mesh produced by the kernel under test (OCCT at its
  default chord tolerance, `1e-4`), with every vertex coloured by its distance
  to the reference: **green = close match, red = divergence**. The shared colour
  scale is printed on the right.

Both meshes come from the *same* OCCT kernel at different tessellation settings,
so the divergence measures sensitivity to the tessellation parameter — a baseline
before real second kernels are added.

## Files

| File | Shows |
| --- | --- |
| `viz_hausdorff_bars.png` | Hausdorff distance per case, sorted. |
| `viz_scatter_rmse_hausdorff.png` | Vertex RMSE vs Hausdorff scatter. |
| `viz_scatter_normal_hausdorff.png` | Normal deviation vs Hausdorff scatter. |
| `viz_category_summary.png` | Pass/fail counts per case category. |
| `viz_category_heatmap.png` | Per-category metric detail. |
| `viz_derived_ness_gradient.png` | All cases ranked by divergence from the reference. |
| `viz_derived_ness_best_worst.png` | Best 20 vs worst 20 by divergence. |
| `viz_category_derived_ness.png` | Mean divergence and pass rate per category. |
| `alignment_summary.txt` | Plain-text summary of the run. |
| `mesh_renders/` | One render per selected case (`mesh_<category>_<case>.png`): all 6 failing cases plus the 6 best-matching passing cases. |
