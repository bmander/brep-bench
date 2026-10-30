#!/usr/bin/env python3
"""The brep-bench runner for Solvent (https://github.com/bmander/geomsolver), the constraint-based
language whose solids cad-kernel builds.

    python3 kernels/solvent/run.py CASE OUT.stl

A case's implementation is a Solvent document, cases/<category>/<name_snake>.sv, which names the
solid it makes in a comment line `// brep-bench solid: NAME`. `solventc` solves the document,
builds the solid with its exact kernel, writes the STL and reports the exact solid's volume and
area. The binary is kernels/solvent/bin/solventc (build.sh makes it) or $SOLVENTC.
"""
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SOLVENTC = os.environ.get("SOLVENTC", os.path.join(HERE, "bin", "solventc"))


def main():
    if sys.argv[1:] == ["--version"]:
        out = subprocess.run([SOLVENTC, "--version"], capture_output=True, text=True).stdout
        commit = ""
        stamp = os.path.join(HERE, "bin", "COMMIT")
        if os.path.exists(stamp):
            commit = " " + open(stamp).read().strip()[:7]
        print(f"Solvent: {out.strip()}{commit}")
        return
    case, out_stl = sys.argv[1], sys.argv[2]
    category, _, name = case.rpartition("/")
    src = os.path.join(HERE, "cases", category, name.replace("-", "_") + ".sv")
    with open(src, encoding="utf-8") as f:
        found = re.search(r"^//\s*brep-bench solid:\s*(\S+)", f.read(), re.M)
    if not found:
        sys.exit(f"{src}: no `// brep-bench solid: NAME` line")
    solid = found.group(1)
    p = subprocess.run([SOLVENTC, src, "--json", "--no-progress", "--stl", out_stl, "--solid", solid],
                       capture_output=True, text=True, cwd=os.path.dirname(src))
    if p.returncode != 0:
        said = [l for l in (p.stderr + p.stdout).splitlines() if l.strip()]
        if any("panicked" in l for l in said):
            sys.stderr.write(p.stderr)
            sys.exit(1)
        reply = {"status": "refused", "reason": (said[-1] if said else "solventc failed")[:300]}
    else:
        doc = json.loads(p.stdout)["documents"][0]
        pos = doc["positions"]
        reply = {"status": "ok", "volume": pos[f"{solid}.volume"], "area": pos[f"{solid}.area"],
                 "faces": {}, "valid": True}
    print(json.dumps(reply))


if __name__ == "__main__":
    main()
