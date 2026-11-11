import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
FIGDIR = ROOT / 'docs' / 'visualizations'
FIGDIR.mkdir(parents=True, exist_ok=True)
MESHDIR = ROOT / 'docs' / 'visualizations' / 'mesh_renders'
RESULTS = ROOT / 'harness' / 'viz' / 'alignment_results.json'



with open(RESULTS) as f:
    results = json.load(f)

# "Derived-ness" visualization: green = matches reference well (least derived),
# red = diverges significantly (most derived)
# Sort by composite score (hausdorff + vertex_rmse normalized)

# Normalize each metric to 0-1
all_hausdorff = np.array([r['hausdorff_rel'] for r in results])
all_rmse = np.array([r['vertex_rmse_rel'] for r in results])
all_normal = np.array([r['normal_deviation_deg'] for r in results])

# Normalize by tolerances
hausdorff_norm = all_hausdorff / 0.05  # tol_hausdorff
rmse_norm = all_rmse / 0.02  # tol_vertex
normal_norm = all_normal / 20.0  # tol_normal

# Composite "derived-ness" score (higher = more derived/divergent)
# Average of the three normalized metrics
derived_score = (hausdorff_norm + rmse_norm + normal_norm) / 3

# Add to results
for i, r in enumerate(results):
    r['derived_score'] = float(derived_score[i])

# Sort by derived score (least derived = green, most derived = red)
results_sorted = sorted(results, key=lambda r: r['derived_score'])

# Create the main "red to green" gradient bar chart
fig, ax = plt.subplots(figsize=(14, 35))
y_pos = np.arange(len(results_sorted))
scores = [r['derived_score'] for r in results_sorted]
cases = [r['case'] for r in results_sorted]

# Custom colormap: green (0) -> yellow (0.5) -> red (1+)
cmap = plt.cm.RdYlGn_r  # Reversed: green=low, red=high
norm = plt.Normalize(vmin=0, vmax=max(2.0, max(scores)))
colors = [cmap(norm(s)) for s in scores]

bars = ax.barh(y_pos, scores, color=colors, edgecolor='white', linewidth=0.3)
ax.set_yticks(y_pos)
ax.set_yticklabels(cases, fontsize=5.5)
ax.set_xlabel('Derived-ness Score (composite of Hausdorff, Vertex RMSE, Normal Deviation normalized to tolerances)', fontsize=10)
ax.set_title('Mesh Derived-ness: Green = Matches Reference (Least Derived) → Red = Diverges (Most Derived)\nScore = Mean of (Hausdorff/0.05, Vertex_RMSE/0.02, Normal/20°)', fontsize=11)
ax.axvline(x=1.0, color='orange', linestyle='--', linewidth=1.5, label='Tolerance threshold (1.0 = at tolerance)')
ax.legend()
plt.tight_layout()
plt.savefig(FIGDIR / 'viz_derived_ness_gradient.png', dpi=150, bbox_inches='tight')
plt.close()

# Also create a compact version: just the top 20 worst and best 20
fig, axes = plt.subplots(1, 2, figsize=(16, 12))

# Best 20 (least derived - green)
best = results_sorted[:20]
y_pos = np.arange(len(best))
scores_b = [r['derived_score'] for r in best]
cases_b = [r['case'] for r in best]
colors_b = [cmap(norm(s)) for s in scores_b]
axes[0].barh(y_pos, scores_b, color=colors_b, edgecolor='white', linewidth=0.5)
axes[0].set_yticks(y_pos)
axes[0].set_yticklabels(cases_b, fontsize=8)
axes[0].set_xlabel('Derived-ness Score')
axes[0].set_title('Best 20: Least Derived (Match Reference Well)', fontsize=11)
axes[0].axvline(x=1.0, color='orange', linestyle='--', linewidth=1)

# Worst 20 (most derived - red)
worst = results_sorted[-20:]
y_pos = np.arange(len(worst))
scores_w = [r['derived_score'] for r in worst]
cases_w = [r['case'] for r in worst]
colors_w = [cmap(norm(s)) for s in scores_w]
axes[1].barh(y_pos, scores_w, color=colors_w, edgecolor='white', linewidth=0.5)
axes[1].set_yticks(y_pos)
axes[1].set_yticklabels(cases_w, fontsize=8)
axes[1].set_xlabel('Derived-ness Score')
axes[1].set_title('Worst 20: Most Derived (Diverge from Reference)', fontsize=11)
axes[1].axvline(x=1.0, color='orange', linestyle='--', linewidth=1)

plt.tight_layout()
plt.savefig(FIGDIR / 'viz_derived_ness_best_worst.png', dpi=150, bbox_inches='tight')
plt.close()

# Create a category-level derived-ness summary
categories = {}
for r in results:
    cat = r['case'].split('\\')[0]
    if cat not in categories:
        categories[cat] = []
    categories[cat].append(r['derived_score'])

cat_names = sorted(categories.keys())
cat_means = [np.mean(categories[c]) for c in cat_names]
cat_maxes = [np.max(categories[c]) for c in cat_names]
cat_pass_rates = [sum(1 for r in results if r['case'].startswith(c+'\\') and r['passed']) / sum(1 for r in results if r['case'].startswith(c+'\\')) for c in cat_names]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 8))

# Mean derived-ness per category
colors_cat = [cmap(norm(m)) for m in cat_means]
y_pos = np.arange(len(cat_names))
ax1.barh(y_pos, cat_means, color=colors_cat, edgecolor='white', linewidth=0.5)
ax1.set_yticks(y_pos)
ax1.set_yticklabels(cat_names, fontsize=9)
ax1.set_xlabel('Mean Derived-ness Score')
ax1.set_title('Mean Derived-ness by Category\n(Green = Category Matches Reference Well)', fontsize=11)
ax1.axvline(x=1.0, color='orange', linestyle='--', linewidth=1)

# Pass rate per category
colors_pr = [plt.cm.RdYlGn(p) for p in cat_pass_rates]
ax2.barh(y_pos, cat_pass_rates, color=colors_pr, edgecolor='white', linewidth=0.5)
ax2.set_yticks(y_pos)
ax2.set_yticklabels(cat_names, fontsize=9)
ax2.set_xlabel('Pass Rate')
ax2.set_title('Pass Rate by Category', fontsize=11)
ax2.set_xlim(0, 1.05)

plt.tight_layout()
plt.savefig(FIGDIR / 'viz_category_derived_ness.png', dpi=150, bbox_inches='tight')
plt.close()

print('Derived-ness visualizations created!')
print('  viz_derived_ness_gradient.png - Full gradient (all 201 cases)')
print('  viz_derived_ness_best_worst.png - Best 20 / Worst 20 comparison')
print('  viz_category_derived_ness.png - Category-level summary')