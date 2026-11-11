#!/usr/bin/env python3
"""What a push needs run again, from what changed since the commit the last results were made at.

    python3 harness/changed.py --base <sha> > plan.json    # what changed since <sha>
    python3 harness/changed.py --full > plan.json          # everything

The plan is JSON: {"full": true} for a run of the whole suite, else {"full": false, "run": {kernel:
[case ids]}, "draw": [case ids] or "all"}, which `run.py --plan` and the workflow read, and "removed": the cases deleted, whose results
`run.py --plan` drops. What
changed decides it:

- a case (cases/<case>.md), added or changed in its header (what is graded: its closed forms,
  whether it is settled): every kernel runs it again. A change to its description alone runs
  nothing: no result holds it, and the site, made again every time, reads it from the file;
- a kernel's implementation of a case: that kernel runs that case again;
- anything else of a kernel's (kernels/<kernel>/): that kernel runs every case again, except its
  workarounds.toml and elegance.toml, which only the site reads;
- a kernel's entry in kernels.json, added or changed in how it is built or run: that kernel runs
  every case again. A change to which kernel is the reference draws every case again; a change to
  what the board only shows (its licence, languages, note, the files its binary's size counts)
  runs nothing;
- how a run is made, measured or graded (harness/run.py, measure.py, mass.py, occt.py, stl.py,
  step.py, suite.py, build.py): the whole suite runs again;
- anything else (the site, the drawings' code, the docs): nothing runs, the site is only made
  again. A change to harness/draw/ draws every case again.

With no base, or a base that is not an ancestor of HEAD (history rewritten), the whole suite runs.
Under GitHub Actions it also writes the step outputs `build` (the kernels to build, space
separated: those with an implementation of a case they are to run), `kernels` (the same, as a JSON
list), `full` ("true" when the whole suite runs), `draw` ("all", case ids, or empty) and `work`
("true" when anything runs or is drawn).
Standard library only.
"""
import argparse
import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from suite import ROOT, cases, implements, kernels  # noqa: E402

WHOLE_SUITE = {"harness/run.py", "harness/measure.py", "harness/mass.py", "harness/occt.py",
               "harness/stl.py", "harness/step.py", "harness/suite.py", "harness/build.py"}
# the code that draws the cases: a change under it draws every case again
REDRAW = "harness/draw/"
# a kernel's flags of its implementations (its workarounds, its elegant ones), which the site reads
# and no run depends on
FLAGS = {"workarounds.toml", "elegance.toml"}
# the fields of a kernel's entry in kernels.json that a run depends on
RUN_FIELDS = ("build", "command", "version", "source", "measured", "step")


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True)


def changed_files(base):
    """The paths changed from `base` to HEAD, or None if `base` cannot be diffed against."""
    if not base or git("merge-base", "--is-ancestor", base, "HEAD").returncode != 0:
        return None
    p = git("diff", "--name-only", "--no-renames", base, "HEAD")
    return p.stdout.split() if p.returncode == 0 else None


def kernels_at(base):
    """The kernels in kernels.json at commit `base`, by name, or None if it cannot be read."""
    p = git("show", f"{base}:kernels.json")
    try:
        return {k["name"]: k for k in json.loads(p.stdout)["kernels"]} if p.returncode == 0 else None
    except (ValueError, KeyError):
        return None


def header(text):
    """A case file's header, the part between its `+++` lines, or None if it has none."""
    parts = text.split("+++", 2)
    return parts[1].strip() if text.startswith("+++") and len(parts) == 3 else None


def redescribed(base, paths):
    """The cases among `paths` whose header is as it was at `base`: only their words changed."""
    out = set()
    for path in paths:
        if not (path.startswith("cases/") and path.endswith(".md")):
            continue
        old = git("show", f"{base}:{path}")
        try:
            with open(os.path.join(ROOT, path), encoding="utf-8") as f:
                new = header(f.read())
        except OSError:
            continue  # removed
        if old.returncode == 0 and new is not None and header(old.stdout) == new:
            out.add(path)
    return out


def implementation(pattern):
    """A kernel's `source` pattern as a regex matching any case's implementation."""
    out = re.escape(pattern)
    for field, part in (("case", ".+"), ("category", "[^/]+"), ("name", "[^/]+"),
                        ("name_snake", "[^/]+")):
        out = out.replace(re.escape("{" + field + "}"), part)
    return re.compile(out + "$")


def plan(paths, results_dir, old_kernels=None, described=()):
    """What to run for `paths` changed; `old_kernels`, the kernels by name before them, is needed
    when kernels.json is among them (without it, the whole suite runs). `described` is the cases
    among `paths` changed only in their description, which need nothing run."""
    runnable = [k for k in kernels() if k.get("command")]
    ids = [c["id"] for c in cases()]
    if paths is None or WHOLE_SUITE & set(paths):
        return {"full": True}
    if "kernels.json" in paths and old_kernels is None:
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
    if "kernels.json" in paths:
        for k in runnable:
            old = old_kernels.get(k["name"])
            if old is None or any(old.get(f) != k.get(f) for f in RUN_FIELDS):
                run[k["name"]] = set(ids)
        reference = lambda ks: [k["name"] for k in ks if k.get("reference")]  # noqa: E731
        if reference(kernels()) != reference(old_kernels.values()):
            redraw_all = True
    for path in paths:
        if path.startswith(REDRAW):
            redraw_all = True
        if path.startswith("cases/") and path.endswith(".md"):
            cid = path[len("cases/"):-len(".md")]
            if path in described:
                pass
            elif cid in ids:
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
            prefix = f"kernels/{k['name']}/"
            if path.startswith(prefix) and path[len(prefix):] not in FLAGS:
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
    old = kernels_at(args.base) if paths and "kernels.json" in paths else None
    described = redescribed(args.base, paths) if paths else ()
    p = plan(paths, os.path.join(ROOT, "results"), old, described)
    json.dump(p, sys.stdout, indent=1)
    print()
    if p["full"]:
        build = [k["name"] for k in kernels() if k.get("command")]
        draw = "all"
        print("the whole suite runs", file=sys.stderr)
    else:
        # a kernel with no implementation of any case it is to run is not built: its runner is
        # asked for nothing, and those cases are only recorded as missing
        by_name = {k["name"]: k for k in kernels()}
        build = [name for name, got in p["run"].items()
                 if any(implements(by_name[name], cid) for cid in got)]
        draw = p["draw"] if p["draw"] == "all" else " ".join(p["draw"])
        for name, got in p["run"].items():
            print(f"{name}: {len(got)} cases", file=sys.stderr)
        if p["removed"]:
            print(f"removed: {' '.join(p['removed'])}", file=sys.stderr)
        if described:
            print(f"{len(described)} cases changed only in their description", file=sys.stderr)
        if not p["run"]:
            print("no case to run", file=sys.stderr)
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as f:
            f.write(f"build={' '.join(build)}\n")
            # the same as JSON, for the workflow's job per kernel
            f.write(f"kernels={json.dumps(build)}\n")
            f.write(f"full={'true' if p['full'] else 'false'}\n")
            f.write(f"draw={draw}\n")
            # a case removed changes the results too: its rows are dropped
            work = bool(build or draw or (not p["full"] and p["removed"]))
            f.write(f"work={'true' if work else 'false'}\n")


if __name__ == "__main__":
    main()
