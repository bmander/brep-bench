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

# Sort by hausdorff distance
results_sorted = sorted(results, key=lambda r: r['hausdorff_rel'])

# Create color map: green for pass, red for fail, gradient by hausdorff
def get_color(r):
    if r['passed']:
        # Green gradient: light green (low hausdorff) to dark green (high but passing)
        h = r['hausdorff_rel']
        # Normalize by max passing hausdorff
        max_pass = max(r['hausdorff_rel'] for r in results if r['passed'])
        norm = min(h / max_pass, 1.0)
        return plt.cm.Greens(0.3 + 0.7 * norm)
    else:
        # Red gradient: light red (low fail) to dark red (high fail)
        h = r['hausdorff_rel']
        max_fail = max(r['hausdorff_rel'] for r in results if not r['passed'])
        min_fail = min(r['hausdorff_rel'] for r in results if not r['passed'])
        norm = (h - min_fail) / (max_fail - min_fail) if max_fail != min_fail else 0.5
        return plt.cm.Reds(0.3 + 0.7 * norm)

colors = [get_color(r) for r in results_sorted]
cases = [r['case'] for r in results_sorted]
hausdorff = [r['hausdorff_rel'] for r in results_sorted]
vertex_rmse = [r['vertex_rmse_rel'] for r in results_sorted]
normal_dev = [r['normal_deviation_deg'] for r in results_sorted]
passed = [r['passed'] for r in results_sorted]

# 1. Horizontal bar chart - Hausdorff distance with pass/fail colors
fig, ax = plt.subplots(figsize=(12, 30))
y_pos = np.arange(len(cases))
bars = ax.barh(y_pos, hausdorff, color=colors, edgecolor='white', linewidth=0.5)
ax.set_yticks(y_pos)
ax.set_yticklabels(cases, fontsize=6)
ax.set_xlabel('Hausdorff Distance (relative)', fontsize=10)
ax.set_title('Mesh Alignment Results: Hausdorff Distance\nGreen = Pass, Red = Fail (darker = worse)', fontsize=12)
ax.axvline(x=0.05, color='orange', linestyle='--', linewidth=1, label='Tolerance (0.05)')
ax.legend()
plt.tight_layout()
plt.savefig(FIGDIR / 'viz_hausdorff_bars.png', dpi=150, bbox_inches='tight')
plt.close()

# 2. Scatter plot: Vertex RMSE vs Hausdorff, colored by pass/fail
fig, ax = plt.subplots(figsize=(10, 8))
pass_h = [r['hausdorff_rel'] for r in results if r['passed']]
pass_v = [r['vertex_rmse_rel'] for r in results if r['passed']]
fail_h = [r['hausdorff_rel'] for r in results if not r['passed']]
fail_v = [r['vertex_rmse_rel'] for r in results if not r['passed']]
ax.scatter(pass_h, pass_v, c='green', alpha=0.6, s=30, label='Pass')
ax.scatter(fail_h, fail_v, c='red', alpha=0.8, s=80, label='Fail', marker='x', linewidths=2)
ax.axvline(x=0.05, color='orange', linestyle='--', linewidth=1)
ax.axhline(y=0.02, color='orange', linestyle='--', linewidth=1)
ax.set_xlabel('Hausdorff Distance (relative)')
ax.set_ylabel('Vertex RMSE (relative)')
ax.set_title('Alignment Quality: Vertex RMSE vs Hausdorff Distance')
ax.legend()
ax.grid(True, alpha=0.3)
plt.tight_layout()
plt.savefig(FIGDIR / 'viz_scatter_rmse_hausdorff.png', dpi=150, bbox_inches='tight')
plt.close()

# 3. Normal deviation vs Hausdorff
fig, ax = plt.subplots(figsize=(10, 8))
pass_h = [r['hausdorff_rel'] for r in results if r['passed']]
pass_n = [r['normal_deviation_deg'] for r in results if r['passed']]
fail_h = [r['hausdorff_rel'] for r in results if not r['passed']]
fail_n = [r['normal_deviation_deg'] for r in results if not r['passed']]
ax.scatter(pass_h, pass_n, c='green', alpha=0.6, s=30, label='Pass')
ax.scatter(fail_h, fail_n, c='red', alpha=0.8, s=80, label='Fail', marker='x', linewidths=2)
ax.axvline(x=0.05, color='orange', linestyle='--', linewidth=1)
ax.axhline(y=20.0, color='orange', linestyle='--', linewidth=1)
ax.set_xlabel('Hausdorff Distance (relative)')
ax.set_ylabel('Normal Deviation (degrees)')
ax.set_title('Alignment Quality: Normal Deviation vs Hausdorff Distance')
ax.legend()
ax.grid(True, alpha=0.3)
plt.tight_layout()
plt.savefig(FIGDIR / 'viz_scatter_normal_hausdorff.png', dpi=150, bbox_inches='tight')
plt.close()

# 4. Category summary - pass/fail counts
categories = {}
for r in results:
    cat = r['case'].split('\\')[0]
    if cat not in categories:
        categories[cat] = {'pass': 0, 'fail': 0, 'total': 0}
    categories[cat]['total'] += 1
    if r['passed']:
        categories[cat]['pass'] += 1
    else:
        categories[cat]['fail'] += 1

cats = sorted(categories.keys())
pass_counts = [categories[c]['pass'] for c in cats]
fail_counts = [categories[c]['fail'] for c in cats]

fig, ax = plt.subplots(figsize=(12, 6))
x = np.arange(len(cats))
width = 0.6
ax.bar(x, pass_counts, width, label='Pass', color='green', alpha=0.7)
ax.bar(x, fail_counts, width, bottom=pass_counts, label='Fail', color='red', alpha=0.7)
ax.set_xticks(x)
ax.set_xticklabels(cats, rotation=45, ha='right', fontsize=9)
ax.set_ylabel('Number of Cases')
ax.set_title('Pass/Fail Count by Category')
ax.legend()
plt.tight_layout()
plt.savefig(FIGDIR / 'viz_category_summary.png', dpi=150, bbox_inches='tight')
plt.close()

# 5. "Derived-ness" heatmap - sorted by category then hausdorff
# Create a grid: rows = categories, columns = cases within category
# Color by hausdorff (green=low, red=high)

fig, axes = plt.subplots(3, 5, figsize=(20, 12))
axes = axes.flatten()
cat_list = sorted(categories.keys())

for idx, cat in enumerate(cat_list):
    if idx >= 15:
        break
    cat_results = [r for r in results if r['case'].startswith(cat + '\\')]
    cat_results.sort(key=lambda r: r['hausdorff_rel'])
    cat_cases = [r['case'].split('\\')[-1] for r in cat_results]
    cat_hausdorff = [r['hausdorff_rel'] for r in cat_results]
    cat_passed = [r['passed'] for r in cat_results]
    
    ax = axes[idx]
    y_pos = np.arange(len(cat_cases))
    colors_cat = ['green' if p else 'red' for p in cat_passed]
    # Adjust alpha by hausdorff
    max_h = max(cat_hausdorff) if cat_hausdorff else 1
    alphas = [0.3 + 0.7 * (h / max_h) for h in cat_hausdorff]
    for i, (y, h, c, a) in enumerate(zip(y_pos, cat_hausdorff, colors_cat, alphas)):
        ax.barh(y, h, color=c, alpha=a, edgecolor='white', linewidth=0.3)
    ax.set_yticks(y_pos)
    ax.set_yticklabels(cat_cases, fontsize=6)
    ax.set_xlabel('Hausdorff', fontsize=8)
    ax.set_title(cat, fontsize=9)
    ax.axvline(x=0.05, color='orange', linestyle='--', linewidth=0.5)

# Hide unused subplots
for idx in range(len(cat_list), 15):
    axes[idx].set_visible(False)

plt.tight_layout()
plt.savefig(FIGDIR / 'viz_category_heatmap.png', dpi=150, bbox_inches='tight')
plt.close()

# 6. Summary stats text
with open(FIGDIR / 'alignment_summary.txt', 'w') as f:
    f.write('BREP Bench - Mesh Alignment Results Summary\n')
    f.write('=' * 50 + '\n\n')
    f.write(f'Total cases: {len(results)}\n')
    f.write(f'Passed: {sum(passed)}\n')
    f.write(f'Failed: {len(results) - sum(passed)}\n')
    f.write(f'Pass rate: {100 * sum(passed) / len(results):.1f}%\n\n')
    f.write('Failed cases:\n')
    for r in results:
        if not r['passed']:
            f.write(f"  {r['case']}: hausdorff={r['hausdorff_rel']:.4f}, vertex_rmse={r['vertex_rmse_rel']:.4f}, normal={r['normal_deviation_deg']:.2f}°\n")
    f.write('\nCategory breakdown:\n')
    for cat in cats:
        c = categories[cat]
        f.write(f"  {cat}: {c['pass']}/{c['total']} passed ({100*c['pass']/c['total']:.0f}%)\n")

print('Visualizations created!')
print('  viz_hausdorff_bars.png - All cases sorted by Hausdorff')
print('  viz_scatter_rmse_hausdorff.png - Vertex RMSE vs Hausdorff')
print('  viz_scatter_normal_hausdorff.png - Normal deviation vs Hausdorff')
print('  viz_category_summary.png - Pass/fail by category')
print('  viz_category_heatmap.png - Per-category detail')
print('  alignment_summary.txt - Text summary')