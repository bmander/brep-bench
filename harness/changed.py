#!/usr/bin/env python3
"""What a push needs run again, from what changed since the commit the last results were made at.

    python3 harness/changed.py --base <sha> > plan.json    # what changed since <sha>
    python3 harness/changed.py --full > plan.json          # everything

The plan is JSON: {"full": true} for a run of the whole suite, else {"full": false, "run": {kernel:
[case ids]}, "draw": [case ids] or "all"}, which `run.py --plan` and the workflow read, and "removed": the cases deleted, whose results
`run.py --plan` drops. What
changed decides it:

- a case (cases/<case>.md): every kernel runs it again;
- a kernel's implementation of a case: that kernel runs that case again;
- anything else of a kernel's (kernels/<kernel>/): that kernel runs every case again, except its
  workarounds.toml, which only the site reads;
- how a run is made or graded (kernels.json, harness/run.py, stl.py, suite.py, build.py): the
  whole suite runs again;
- anything else (the site, the drawings' code, the docs): nothing runs, the site is only made
  again. A change to harness/draw.py draws every case again.

With no base, or a base that is not an ancestor of HEAD (history rewritten), the whole suite runs.
Under GitHub Actions it also writes the step outputs `build` (the kernels to build, space
separated), `draw` ("all", case ids, or empty) and `work` ("true" when anything runs or is drawn).
Standard library only.
"""
import argparse
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from suite import ROOT, cases, kernels  # noqa: E402

WHOLE_SUITE = {"kernels.json", "harness/run.py", "harness/stl.py", "harness/suite.py",
               "harness/build.py"}
REDRAW = {"harness/draw.py", "harness/stl.py"}
# a kernel's flags of its workarounds, which the site reads and no run depends on
FLAGS = "workarounds.toml"


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True)


def changed_files(base):
    """The paths changed from `base` to HEAD, or None if `base` cannot be diffed against."""
    if not base or git("merge-base", "--is-ancestor", base, "HEAD").returncode != 0:
        return None
    p = git("diff", "--name-only", "--no-renames", base, "HEAD")
    return p.stdout.split() if p.returncode == 0 else None


def implementation(pattern):
    """A kernel's `source` pattern as a regex matching any case's implementation."""
    out = re.escape(pattern)
    for field, part in (("case", ".+"), ("category", "[^/]+"), ("name", "[^/]+"),
                        ("name_snake", "[^/]+")):
        out = out.replace(re.escape("{" + field + "}"), part)
    return re.compile(out + "$")


def plan(paths, results_dir):
    runnable = [k for k in kernels() if k.get("command")]
    ids = [c["id"] for c in cases()]
    if paths is None or WHOLE_SUITE & set(paths):
        return {"full": True}
    # each kernel's implementation of each case, by its path
    owner = {}
    for k in runnable:
        for cid in ids:
            category, _, name = cid.rpartition("/")
            if k.get("source"):
                owner[k["source"].format(case=cid, category=category, name=name,
                                         name_snake=name.replace("-", "_"))] = (k["name"], cid)
    run = {}
    removed = set()
    redraw_all = False
    for path in paths:
        if path in REDRAW:
            redraw_all = True
        if path.startswith("cases/") and path.endswith(".md"):
            cid = path[len("cases/"):-len(".md")]
            if cid in ids:
                for k in runnable:
                    run.setdefault(k["name"], set()).add(cid)
            else:
                removed.add(cid)
            continue
        if path in owner:
            name, cid = owner[path]
            run.setdefault(name, set()).add(cid)
            continue
        for k in runnable:
            if path.startswith(f"kernels/{k['name']}/") and path != f"kernels/{k['name']}/{FLAGS}":
                # an implementation of a case that is gone needs nothing run
                if not (k.get("source") and implementation(k["source"]).match(path)):
                    run[k["name"]] = set(ids)
    # a kernel with no results yet to add to runs every case
    for name in run:
        if not os.path.exists(os.path.join(results_dir, name + ".json")):
            run[name] = set(ids)
    drawn = sorted({cid for got in run.values() for cid in got})
    return {"full": False, "run": {k: sorted(v) for k, v in sorted(run.items())},
            "draw": "all" if redraw_all else drawn, "removed": sorted(removed)}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", help="the commit the last results were made at")
    ap.add_argument("--full", action="store_true", help="run the whole suite")
    args = ap.parse_args()
    paths = None if args.full else changed_files(args.base)
    p = plan(paths, os.path.join(ROOT, "results"))
    json.dump(p, sys.stdout, indent=1)
    print()
    if p["full"]:
        build = [k["name"] for k in kernels() if k.get("command")]
        draw = "all"
        print("the whole suite runs", file=sys.stderr)
    else:
        build = list(p["run"])
        draw = p["draw"] if p["draw"] == "all" else " ".join(p["draw"])
        for name, got in p["run"].items():
            print(f"{name}: {len(got)} cases", file=sys.stderr)
        if p["removed"]:
            print(f"removed: {' '.join(p['removed'])}", file=sys.stderr)
        if not p["run"]:
            print("no case to run", file=sys.stderr)
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as f:
            f.write(f"build={' '.join(build)}\n")
            f.write(f"draw={draw}\n")
            # a case removed changes the results too: its rows are dropped
            work = bool(build or draw or (not p["full"] and p["removed"]))
            f.write(f"work={'true' if work else 'false'}\n")


if __name__ == "__main__":
    main()
