#!/usr/bin/env python3
"""Run every case against every kernel and grade the results.

    python3 harness/run.py                     # every kernel in kernels.json, every case
    python3 harness/run.py --kernel arris --case boolean/
    python3 harness/run.py --timeout 120

A kernel's runner is asked for each case it has an implementation of (SPEC.md); a case it has none
of is missing. Writes results/<kernel>.json after a run of the whole suite (not of a part,
`--case`), and keeps each result's triangles as results/meshes/<kernel>/<case>.stl for the drawings
and the viewer. Standard library only.
"""
import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stl  # noqa: E402
from suite import ROOT, cases, kernels, source  # noqa: E402

# How near a measure must be to the expected value, relative to it.
EXACT = 1e-6
# How near the volume of the runner's own triangles must be: a mesh is within its sag of the
# solid, so this only catches a mesh of some other solid.
MESHED = 2e-2
# How near the measures must be for a kernel with no exact measures of its own, which the harness
# then takes from the runner's triangles (`"measured": "mesh"`, SPEC.md).
FROM_MESH = 1e-3


def near(got, want, rel):
    return got is not None and abs(got - want) <= rel * max(abs(want), 1e-12)


def grade(case, reply, mesh):
    """The outcome of one run: pass (built for an unsettled case), refused, wrong (with why) or
    crashed."""
    status = reply.get("status")
    if status == "refused":
        return "refused", reply.get("reason", "")
    if status != "ok":
        return "crashed", f"status {status!r}"
    why = []
    exp = case["expect"]
    if exp is None:
        # unsettled: no answer to grade against, only whether the solid is sound
        if mesh is None:
            return "wrong", "no triangles written"
        if not mesh["closed"]:
            return "wrong", f"triangles not closed ({mesh['unmatched_edges']} edges unmatched)"
        return "built", ""
    from_mesh = reply.get("measured") == "mesh"
    for key in ("volume", "area"):
        got, want = reply.get(key), exp[key]["value"]
        if from_mesh:
            got = mesh and mesh[key]
        if not near(got, want, FROM_MESH if from_mesh else EXACT):
            why.append(f"{key} {got} against {want:.12g}")
    if mesh is None:
        why.append("no triangles written")
    else:
        if not mesh["closed"]:
            why.append(f"triangles not closed ({mesh['unmatched_edges']} edges unmatched)")
        if not near(mesh["volume"], exp["volume"]["value"], MESHED):
            why.append(f"triangles' volume {mesh['volume']:.6g}")
    return ("pass", "") if not why else ("wrong", "; ".join(why))


def why_crashed(stderr, code):
    """What a crashed runner said last: a Rust panic's place and message, else its last line."""
    lines = [l for l in stderr.strip().splitlines() if l.strip() and not l.startswith("note:")]
    for i, line in enumerate(lines):
        if "panicked at" in line:
            # the place in the kernel's source, not where cargo keeps it
            said = re.sub(r"\S*/\.cargo/(git/checkouts|registry/src)/[^/]+/[^/]+/", "",
                          " ".join(lines[i:i + 2]))
            return re.sub(r"^thread '[^']*'( \(\d+\))? ", "", said)[:300]
    return (lines[-1] if lines else f"exit {code}")[:300]


def run_one(command, case, timeout, keep):
    with tempfile.TemporaryDirectory() as tmp:
        out_stl = os.path.join(tmp, "result.stl")
        start = time.monotonic()
        try:
            env = dict(os.environ, RUST_BACKTRACE="0")
            p = subprocess.run(command + [case["id"], out_stl], capture_output=True, text=True,
                               timeout=timeout, cwd=ROOT, env=env)
        except subprocess.TimeoutExpired:
            return "timeout", f"over {timeout} s", None, None, timeout
        seconds = time.monotonic() - start
        if p.returncode != 0:
            return "crashed", why_crashed(p.stderr or "", p.returncode), None, None, seconds
        try:
            reply = json.loads(p.stdout.strip().splitlines()[-1])
        except (ValueError, IndexError):
            return "crashed", "no JSON reply", None, None, seconds
        mesh = None
        if reply.get("status") == "ok" and os.path.exists(out_stl):
            try:
                mesh = stl.check(stl.read(out_stl))
            except (ValueError, OSError) as e:
                return "wrong", f"unreadable STL: {e}", reply, None, seconds
            # kept for the drawings and the viewer (results/meshes, not committed)
            os.makedirs(os.path.dirname(keep), exist_ok=True)
            shutil.copyfile(out_stl, keep)
        return (*grade(case, reply, mesh), reply, mesh, seconds)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--kernel", action="append", help="kernel name (repeatable)")
    ap.add_argument("--case", default="", help="run the cases whose id starts with this")
    ap.add_argument("--timeout", type=float, default=60)
    args = ap.parse_args()
    chosen = [k for k in kernels() if not args.kernel or k["name"] in args.kernel]
    os.makedirs(os.path.join(ROOT, "results"), exist_ok=True)
    for k in chosen:
        if not k.get("command"):
            continue
        rows = []
        tally = {}
        for c in cases(args.case):
            keep = os.path.join(ROOT, "results", "meshes", k["name"], c["id"] + ".stl")
            if os.path.exists(keep):
                os.remove(keep)
            src = source(k, c["id"])
            if src and not os.path.exists(src):
                outcome, why, reply, mesh, seconds = "missing", "no implementation", None, None, 0.0
            else:
                outcome, why, reply, mesh, seconds = run_one(k["command"], c, args.timeout, keep)
            tally[outcome] = tally.get(outcome, 0) + 1
            rows.append({
                "case": c["id"], "outcome": outcome, "why": why, "seconds": round(seconds, 3),
                "measured": (reply or {}).get("measured", "exact"),
                "volume": (reply or {}).get("volume"), "area": (reply or {}).get("area"),
                "faces": (reply or {}).get("faces"), "mesh": mesh,
            })
            print(f"{k['name']:<14} {outcome:<8} {c['id']:<44} {why[:90]}", flush=True)
        version = None
        if k.get("version"):
            try:
                version = subprocess.run(k["version"], capture_output=True, text=True,
                                         cwd=ROOT, timeout=60).stdout.strip() or None
            except (OSError, subprocess.TimeoutExpired):
                pass
        result = {"kernel": k["name"], "version": version,
                  "measured": k.get("measured", "exact"),
                  "run": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                  "tally": tally, "cases": rows}
        # a part of the suite is a check in hand, not a result: only a whole run is recorded
        if not args.case:
            with open(os.path.join(ROOT, "results", k["name"] + ".json"), "w") as f:
                json.dump(result, f, indent=1)
                f.write("\n")
        print(f"{k['name']}: {tally}")


if __name__ == "__main__":
    main()
