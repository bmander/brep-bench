import sys
import json
import numpy as np
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from harness.align import grade_alignment

mesh_dir = ROOT / 'results' / 'meshes' / 'occt'
ref_dir = ROOT / 'results' / 'refs'
OUT = ROOT / 'harness' / 'viz' / 'alignment_results.json'

all_cases = []
for cat_dir in mesh_dir.iterdir():
    if cat_dir.is_dir():
        for mesh_file in cat_dir.glob('*.stl'):
            rel = mesh_file.relative_to(mesh_dir)
            ref_file = ref_dir / rel
            if ref_file.exists():
                all_cases.append(str(rel.with_suffix('')))

print(f'Total cases: {len(all_cases)}')

results = []
for case in all_cases:
    r = grade_alignment(str(mesh_dir / f'{case}.stl'), str(ref_dir / f'{case}.stl'), tol_vertex=0.02, tol_hausdorff=0.05, tol_normal=20.0)
    # Convert numpy types to Python types
    def convert(obj):
        if isinstance(obj, (np.bool_, np.integer, np.floating)):
            return obj.item()
        elif isinstance(obj, dict):
            return {k: convert(v) for k, v in obj.items()}
        elif isinstance(obj, list):
            return [convert(v) for v in obj]
        return obj
    r = convert(r)
    r['case'] = case
    results.append(r)
    status = 'PASS' if r['passed'] else 'FAIL'
    print(f'{case}: {status}, hausdorff={r["hausdorff_rel"]:.4f}, vertex_rmse={r["vertex_rmse_rel"]:.4f}, normal={r["normal_deviation_deg"]:.2f}')

with open(OUT, 'w') as f:
    json.dump(results, f, indent=2)
print(f'\nSaved to {OUT.relative_to(ROOT)}')