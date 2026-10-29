#!/usr/bin/env python3
"""Build every adapter in adapters.json that has a `build` command (or those named).

    python3 harness/build.py                 # all
    python3 harness/build.py truck arris     # these

An adapter that fails to build is reported and left out; the others still build. Standard library
only.
"""
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main():
    with open(os.path.join(ROOT, "adapters.json")) as f:
        adapters = json.load(f)["adapters"]
    wanted = set(sys.argv[1:])
    failed = []
    for a in adapters:
        if not a.get("build") or (wanted and a["name"] not in wanted):
            continue
        print(f"== {a['name']}: {' '.join(a['build'])}", flush=True)
        if subprocess.run(a["build"], cwd=ROOT).returncode != 0:
            failed.append(a["name"])
    if failed:
        print(f"failed to build: {', '.join(failed)}")


if __name__ == "__main__":
    main()
