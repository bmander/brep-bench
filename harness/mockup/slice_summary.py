# Mock-up (the hidden slice): what each draw of harness/slice.py --json came to, case by case.
# python3 harness/mockup/slice_summary.py DIR, DIR holding the --json files.
import collections
import glob
import json
import sys

rows = []
for f in sorted(glob.glob(sys.argv[1] + "/*.json")):
    rows += json.load(open(f))
by = collections.Counter()
cases = collections.defaultdict(lambda: collections.Counter())
grades = collections.Counter()
for r in rows:
    g = r["grade"].split(" ")[0]
    grades[g] += 1
    for d in r.get("draws", []):
        res = d["result"]
        if "refused" in res:
            k = "refused"
        elif all(ok for ok, _ in res.values()):
            k = "ok"
        else:
            k = "mismatch:" + ",".join(n for n, (ok, _) in res.items() if not ok)
        by[(g, k.split(":")[0])] += 1
        cases[r["case"]][k] += 1
print("whole-solid grades:", dict(grades))
print("draws by grade and slice outcome:")
for (g, k), n in sorted(by.items()):
    print(f"  {g:10} {k:9} {n}")
print("cases with any slice not ok:")
for r in rows:
    c = cases[r["case"]]
    if any(k != "ok" for k in c):
        print(f"  {r['case']:52} {r['grade'][:38]:38} {dict(c)}")
print("mismatch details:")
for r in rows:
    for i, d in enumerate(r.get("draws", [])):
        res = d["result"]
        if "refused" not in res and not all(ok for ok, _ in res.values()):
            print(f"  {r['case']} draw {i}: " + "; ".join(f"{n} {det}" for n, (ok, det) in res.items() if not ok)[:300])
