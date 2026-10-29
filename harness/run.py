#!/usr/bin/env python3
"""Run every case against every adapter and grade the results.

    python3 harness/run.py                     # every adapter in adapters.json, every case
    python3 harness/run.py --adapter cad-kernel --case boolean/
    python3 harness/run.py --timeout 120

Writes results/<adapter>.json after a run of the whole suite (not of a part, `--case`). Standard
library only.
"""
import argparse
import json
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stl  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# How near a measure must be to the expected value, relative to it.
EXACT = 1e-6
# How near the volume of the adapter's own triangles must be: a mesh is within its sag of the
# solid, so this only catches a mesh of some other solid.
MESHED = 2e-2
# How near the measures must be for a kernel with no exact measures of its own, which the harness
# then takes from the adapter's triangles (`"measured": "mesh"`, SPEC.md).
FROM_MESH = 1e-3


def cases(prefix):
    out = []
    for dirpath, _, files in os.walk(os.path.join(ROOT, "cases")):
        for f in sorted(files):
            if f.endswith(".json"):
                path = os.path.join(dirpath, f)
                with open(path) as fh:
                    c = json.load(fh)
                if c["id"].startswith(prefix):
                    out.append((path, c))
    return sorted(out, key=lambda pc: pc[1]["id"])


def near(got, want, rel):
    return got is not None and abs(got - want) <= rel * max(abs(want), 1e-12)


def grade(case, reply, mesh):
    """The outcome of one run: pass, refused, wrong (with why) or crashed."""
    status = reply.get("status")
    if status == "refused":
        return "refused", reply.get("reason", "")
    if status != "ok":
        return "crashed", f"status {status!r}"
    why = []
    exp = case["expect"]
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


def run_one(command, case_path, timeout):
    with tempfile.TemporaryDirectory() as tmp:
        out_stl = os.path.join(tmp, "result.stl")
        start = time.monotonic()
        try:
            p = subprocess.run(command + [case_path, out_stl], capture_output=True, text=True,
                               timeout=timeout, cwd=ROOT)
        except subprocess.TimeoutExpired:
            return "timeout", f"over {timeout} s", None, None, timeout
        seconds = time.monotonic() - start
        if p.returncode != 0:
            tail = (p.stderr or "").strip().splitlines()[-1:] or [f"exit {p.returncode}"]
            return "crashed", tail[0][:300], None, None, seconds
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
        return (*grade(json.load(open(case_path)), reply, mesh), reply, mesh, seconds)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--adapter", action="append", help="adapter name (repeatable)")
    ap.add_argument("--case", default="", help="run the cases whose id starts with this")
    ap.add_argument("--timeout", type=float, default=60)
    args = ap.parse_args()
    with open(os.path.join(ROOT, "adapters.json")) as f:
        adapters = json.load(f)["adapters"]
    if args.adapter:
        adapters = [a for a in adapters if a["name"] in args.adapter]
    os.makedirs(os.path.join(ROOT, "results"), exist_ok=True)
    for a in adapters:
        if not a.get("command"):
            continue
        command = a["command"]
        rows = []
        tally = {}
        for path, c in cases(args.case):
            outcome, why, reply, mesh, seconds = run_one(command, path, args.timeout)
            tally[outcome] = tally.get(outcome, 0) + 1
            rows.append({
                "case": c["id"], "outcome": outcome, "why": why, "seconds": round(seconds, 3),
                "measured": (reply or {}).get("measured", "exact"),
                "volume": (reply or {}).get("volume"), "area": (reply or {}).get("area"),
                "faces": (reply or {}).get("faces"), "mesh": mesh,
            })
            print(f"{a['name']:<14} {outcome:<8} {c['id']:<44} {why[:90]}", flush=True)
        version = None
        if a.get("version"):
            try:
                version = subprocess.run(a["version"], capture_output=True, text=True,
                                         cwd=ROOT, timeout=60).stdout.strip() or None
            except (OSError, subprocess.TimeoutExpired):
                pass
        result = {"adapter": a["name"], "version": version,
                  "measured": a.get("measured", "exact"),
                  "run": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                  "tally": tally, "cases": rows}
        # a part of the suite is a check in hand, not a result: only a whole run is recorded
        if not args.case:
            with open(os.path.join(ROOT, "results", a["name"] + ".json"), "w") as f:
                json.dump(result, f, indent=1)
                f.write("\n")
        print(f"{a['name']}: {tally}")


if __name__ == "__main__":
    main()
