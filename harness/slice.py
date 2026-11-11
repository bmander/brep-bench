#!/usr/bin/env python3
"""Mock-up: the hidden slice. Not part of the graded run.

    python3 harness/slice.py --kernel cadrs --draws 8 swept/drum-pushed-through-plate swept/...
    python3 harness/slice.py --kernel cadrs --draws 8 --seed nightly-2026-11-11 swept/

Each draw hands the kernel's runner a plane (env BREP_SLICE = "px,py,pz,nx,ny,nz"); the runner
builds the case, writes the solid as OUT.step as usual, and the solid cut by the plane as
OUT.step.above.step and OUT.step.below.step (its own boolean, its own STEP writer). Nothing a
kernel chose is an answer: the two sides must add back up to the whole, as harness/mass.py
measures each of the three files.

- **closed**: each side's file holds a closed solid, right side out (harness/step.py's B-rep or
  faceted, and measured);
- **volume**: the sides' volumes sum to the whole's;
- **centroid**: their centres of gravity, weighted by volume, are the whole's;
- **inertia**: their inertias, carried to the whole's centre of gravity, sum to the whole's;
- **area**: their areas sum to the whole's and the cut face twice over, the cut face being the
  side's faces on the plane, the same area on both sides;
- **sides**: every vertex of a side lies on its side of the plane.

The plane's point is drawn from the middle half of the whole solid's box (each coordinate), its
normal uniformly over the sphere, from `random.Random("<seed>/<case>/<i>")`. Tolerances are the
grade's 1e-6, relative to the whole's volume, area and size, and its inertia's largest entry.
"""
import argparse
import json
import math
import os
import random
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mass  # noqa: E402
import step  # noqa: E402
from run import last_json  # noqa: E402
from suite import ROOT, cases, implements, kernels  # noqa: E402

TOL = 1e-6


def vertices(text):
    m = mass.Model(text)
    out = []
    for r, e in m.entities.items():
        if e.name == "VERTEX_POINT":
            out.append(m.point(m.one(r)[1][1]))
    return m, out


def cut_area(text, p, n, size):
    """The area of the faces of the file's solid lying on the plane through p along n."""
    m = mass.Model(text)
    faces, _ = mass.faces_of(m)
    centre, scale = mass.bounding(m)
    total, count = 0.0, 0
    for f, _sense in faces:
        fn, fa = m.one(f)
        if fn not in mass.stepfile.FACES:
            continue
        s = m.surface(fa[2])
        if not isinstance(s, mass.Plane):
            continue
        if abs(abs(mass.dot(s.Z, n)) - 1) > 1e-9 or abs(mass.dot(mass.sub(s.O, p), n)) > 1e-7 * size:
            continue
        loops = mass.face_loops(m, f, s, scale)
        standard, _ = mass.Face(s, loops, fa[3] != "F", centre, scale).measure()
        total += standard[1]
        count += 1
    return total, count


def draw_plane(rng, lo, hi):
    p = [lo[i] + (0.25 + 0.5 * rng.random()) * (hi[i] - lo[i]) for i in range(3)]
    while True:
        n = [rng.gauss(0, 1) for _ in range(3)]
        r = math.sqrt(sum(x * x for x in n))
        if r > 1e-6:
            return p, [x / r for x in n]


def run(command, cid, work, plane, timeout):
    out_stl = os.path.join(work, "out.stl")
    out_step = os.path.join(work, "out.step")
    for f in os.listdir(work):
        os.remove(os.path.join(work, f))
    env = dict(os.environ)
    if plane:
        env["BREP_SLICE"] = ",".join(repr(x) for x in plane[0] + plane[1])
    try:
        p = subprocess.run(command + [cid, out_stl, out_step], capture_output=True, text=True,
                           timeout=timeout, cwd=ROOT, env=env)
    except subprocess.TimeoutExpired:
        return {"error": f"timeout {timeout:g} s"}
    lines = [json.loads(l) for l in p.stdout.splitlines() if l.startswith("{")]
    slice_said = next((l["slice"] for l in lines if "slice" in l), None)
    reply = last_json(p.stdout) or {}
    files = {}
    for name, path in (("whole", out_step), ("above", out_step + ".above.step"),
                       ("below", out_step + ".below.step")):
        if os.path.exists(path):
            with open(path, encoding="latin-1") as f:
                files[name] = f.read()
    return {"reply": reply, "slice": slice_said, "files": files, "code": p.returncode,
            "stderr": p.stderr[-400:]}


def occt_sliced(whole_text, plane, work):
    """The two sides of the solid in `whole_text` cut by `plane`, by Open CASCADE (no runner
    change at all: the harness cuts the kernel's file): {"above": text, "below": text}, or
    {"refused": why}."""
    from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
    from OCP.BRepCheck import BRepCheck_Analyzer
    from OCP.IFSelect import IFSelect_RetDone
    from OCP.STEPControl import STEPControl_AsIs, STEPControl_Reader, STEPControl_Writer
    from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt
    src = os.path.join(work, "whole.step")
    with open(src, "w", encoding="latin-1") as f:
        f.write(whole_text)
    r = STEPControl_Reader()
    if r.ReadFile(src) != IFSelect_RetDone:
        return {"refused": "OCCT did not read the file"}
    r.TransferRoots()
    shape = r.OneShape()
    p, n = plane
    L = 1e3
    # a sketch x axis square to n
    a = [1.0, 0.0, 0.0] if abs(n[0]) < 0.9 else [0.0, 1.0, 0.0]
    d = sum(a[i] * n[i] for i in range(3))
    x = [a[i] - d * n[i] for i in range(3)]
    s = math.sqrt(sum(v * v for v in x))
    x = [v / s for v in x]
    y = [n[1] * x[2] - n[2] * x[1], n[2] * x[0] - n[0] * x[2], n[0] * x[1] - n[1] * x[0]]
    out = {}
    for side, sign in (("above", 1), ("below", -1)):
        m = [sign * v for v in n]
        ys = [sign * v for v in y]
        corner = [p[i] - L / 2 * x[i] - L / 2 * ys[i] for i in range(3)]
        box = BRepPrimAPI_MakeBox(gp_Ax2(gp_Pnt(*corner), gp_Dir(*m), gp_Dir(*x)), L, L, L).Shape()
        op = BRepAlgoAPI_Common(shape, box)
        if not op.IsDone():
            return {"refused": f"{side}: OCCT's common failed"}
        piece = op.Shape()
        if not BRepCheck_Analyzer(piece).IsValid():
            return {"refused": f"{side}: OCCT's common is not valid"}
        w = STEPControl_Writer()
        path = os.path.join(work, f"{side}.step")
        # (OCCT prints its writer's statistics to file descriptor 1: sent to nothing)
        sys.stdout.flush()
        saved, null = os.dup(1), os.open(os.devnull, os.O_WRONLY)
        os.dup2(null, 1)
        try:
            w.Transfer(piece, STEPControl_AsIs)
            w.Write(path)
        finally:
            os.dup2(saved, 1)
            os.close(saved)
            os.close(null)
        with open(path, encoding="latin-1") as f:
            out[side] = f.read()
    return out


def measured(text):
    with tempfile.NamedTemporaryFile("w", suffix=".step", delete=False, encoding="latin-1") as f:
        f.write(text)
        path = f.name
    try:
        made = step.read(path)
        verdict = step.verdict(made)
    except ValueError as e:
        verdict = f"unread: {e}"
    m = mass.read(path)
    os.remove(path)
    return verdict, m


def combine(parts):
    """Volume, centroid and inertia (about the centroid) of solids given as (V, g, I)."""
    V = sum(v for v, _, _ in parts)
    g = [sum(v * c[i] for v, c, _ in parts) / V for i in range(3)]
    I = [[0.0] * 3 for _ in range(3)]
    for v, c, Ii in parts:
        d = [c[i] - g[i] for i in range(3)]
        dd = sum(x * x for x in d)
        for i in range(3):
            for j in range(3):
                I[i][j] += Ii[i][j] + v * ((dd if i == j else 0.0) - d[i] * d[j])
    return V, g, I


def check(whole_text, sides, plane):
    """The checks of one draw: {name: (ok, detail)}."""
    p, n = plane
    _, wv = vertices(whole_text)
    lo = [min(q[i] for q in wv) for i in range(3)]
    hi = [max(q[i] for q in wv) for i in range(3)]
    size = math.dist(lo, hi)
    out = {}
    _, w = measured(whole_text)
    if "volume" not in w:
        return {"whole": (False, w)}
    got = {}
    for side in ("above", "below"):
        text = sides.get(side)
        if text is None:
            out["closed"] = (False, f"{side}: no file")
            return out
        verdict, m = measured(text)
        if verdict not in ("brep", "faceted") or "volume" not in m or m["volume"] <= 0:
            out["closed"] = (False, f"{side}: {verdict}, {m.get('error') or m.get('invalid') or m.get('volume')}")
            return out
        got[side] = (m, text)
    out["closed"] = (True, "")
    a, b = got["above"][0], got["below"][0]
    V, g, I = combine([(x["volume"], x["centroid"], x["inertia"]) for x in (a, b)])
    dv = (V - w["volume"]) / w["volume"]
    out["volume"] = (abs(dv) <= TOL, f"{dv:+.1e}")
    dg = math.dist(g, w["centroid"]) / size
    out["centroid"] = (dg <= TOL, f"{dg:.1e}")
    big = max(abs(x) for row in w["inertia"] for x in row)
    dI = max(abs(I[i][j] - w["inertia"][i][j]) for i in range(3) for j in range(3)) / big
    out["inertia"] = (dI <= TOL, f"{dI:.1e}")
    try:
        ca, na = cut_area(got["above"][1], p, n, size)
        cb, nb = cut_area(got["below"][1], p, n, size)
        da = (a["area"] + b["area"] - w["area"] - ca - cb) / w["area"]
        dc = (ca - cb) / w["area"]
        ok = abs(da) <= TOL and abs(dc) <= TOL and na > 0 and nb > 0
        out["area"] = (ok, f"{da:+.1e} (cut {ca:.6g}/{cb:.6g}, {na}/{nb} faces)")
    except Exception as e:  # noqa: BLE001
        out["area"] = (False, f"cut face not measured: {type(e).__name__}: {e}")
    worst = 0.0
    for side, sign in (("above", 1), ("below", -1)):
        _, vs = vertices(got[side][1])
        for q in vs:
            worst = max(worst, -sign * sum((q[i] - p[i]) * n[i] for i in range(3)) / size)
    out["sides"] = (worst <= TOL, f"{worst:.1e}")
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cases", nargs="+")
    ap.add_argument("--kernel", required=True)
    ap.add_argument("--draws", type=int, default=6)
    ap.add_argument("--seed", default="slice-mock")
    ap.add_argument("--timeout", type=float, default=120)
    ap.add_argument("--json")
    ap.add_argument("--slicer", choices=["kernel", "occt"], default="kernel",
                    help="who cuts: the kernel's runner (BREP_SLICE), or OCCT in the harness")
    a = ap.parse_args()
    k = next(k for k in kernels() if k["name"] == a.kernel)
    command = k["command"]
    chosen = [c for c in cases() if any(c["id"] == x or (x.endswith("/") and c["id"].startswith(x))
                                       for x in a.cases) and implements(k, c["id"])]
    rows = []
    work = tempfile.mkdtemp(prefix="slice-")
    for c in chosen:
        base = run(command, c["id"], work, None, a.timeout)
        whole = base.get("files", {}).get("whole")
        grade = "?"
        if whole is None:
            grade = f"no STEP: {base.get('error') or base['reply'].get('status')} {base.get('reply', {}).get('reason', '')[:80]}"
            print(f"{c['id']}: {grade}", flush=True)
            rows.append({"case": c["id"], "grade": grade})
            continue
        _, w = measured(whole)
        exp = c["expect"]
        if exp and "volume" in w:
            dv = w["volume"] / exp["volume"]["value"] - 1
            da = w["area"] / exp["area"]["value"] - 1
            grade = ("pass" if abs(dv) <= TOL and abs(da) <= TOL else "wrong") + f" (V {dv:+.1e}, A {da:+.1e})"
        elif "volume" in w:
            grade = "built (unsettled)"
        else:
            grade = f"unmeasured: {w}"
        print(f"{c['id']}: {grade}", flush=True)
        _, wv = vertices(whole)
        lo = [min(q[i] for q in wv) for i in range(3)]
        hi = [max(q[i] for q in wv) for i in range(3)]
        draws = []
        for i in range(a.draws):
            rng = random.Random(f"{a.seed}/{c['id']}/{i}")
            plane = draw_plane(rng, lo, hi)
            if a.slicer == "occt":
                cut_ = occt_sliced(whole, plane, work)
                r = ({"slice": cut_["refused"]} if "refused" in cut_ else
                     {"slice": "ok", "files": {"whole": whole, **cut_}})
            else:
                r = run(command, c["id"], work, plane, a.timeout)
            if r.get("error") or r["slice"] != "ok":
                res = {"refused": r.get("error") or r["slice"] or f"no slice (exit {r['code']}) {r['stderr'][-200:]}"}
                print(f"   draw {i}: slice refused: {str(res['refused'])[:140]}", flush=True)
            else:
                try:
                    res = {k2: list(v) for k2, v in check(r["files"]["whole"], r["files"], plane).items()}
                except Exception as e:  # noqa: BLE001
                    res = {"harness": [False, f"{type(e).__name__}: {e}"]}
                bad = [f"{k2} {d}" for k2, (ok, d) in res.items() if not ok]
                print(f"   draw {i}: " + ("ok" if not bad else "FAIL " + "; ".join(bad)), flush=True)
            draws.append({"plane": plane, "result": res})
        rows.append({"case": c["id"], "grade": grade, "draws": draws})
    if a.json:
        with open(a.json, "w") as f:
            json.dump(rows, f, indent=1)


if __name__ == "__main__":
    main()
