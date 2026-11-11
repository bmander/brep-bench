"""Control for the slice mock-up: cadrs's swept cases placed aslant, and at other numbers.

A placed variant must measure as its case does (a rigid motion moves neither volume nor area); a
resized one is graded on its volume, from the case's own closed form at the new numbers.
"""
import json
import math
import os
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(ROOT, "harness"))
import mass  # noqa: E402
import step  # noqa: E402
from suite import cases  # noqa: E402

RUN = os.path.join(ROOT, "kernels/cadrs/target/release/brep-bench-cadrs")
pi = math.pi


def lens(R, r, d):
    return (r * r * math.acos((d * d + r * r - R * R) / (2 * d * r))
            + R * R * math.acos((d * d + R * R - r * r) / (2 * d * R))
            - 0.5 * math.sqrt((-d + r + R) * (d + r - R) * (d - r + R) * (d + r + R)))


def ellipsoid(a, b, c, d):
    n = math.sqrt(sum(x * x for x in d))
    u = [x / n for x in d]
    return 3600 - 4 * pi * math.sqrt(b * b * c * c * u[0] ** 2 + a * a * c * c * u[1] ** 2
                                     + a * a * b * b * u[2] ** 2) * n / abs(d[2])


def ring_slid(R, r, L):
    return 2 * pi * r * R * R + pi * pi * R * r * r + 4 * pi * r ** 3 / 3 + 2 * L * (2 * r * R + pi * r * r / 2)


RESIZED = {
    "box-slid-aslant": 7 * 9 * 3 + 5 * 9 * 3 + 4 * 3 * 7 + 6 * 7 * 9,
    "ball-swung-groove": 24 * 24 * 8 - 0.5 * (pi * 2.5 ** 2 * 10 * math.radians(70) + 4 * pi * 2.5 ** 3 / 3),
    "ring-lying-flat-turned-whole": 2 * pi * 6.5 * (4 * 0.9 * 2.2 + pi * 0.9 ** 2),
    "ring-slid-flat-past-its-hole": ring_slid(5, 1.5, 13),
    "cube-spun-on-its-diagonal-inside-a-block": 216 - 8 * pi / math.sqrt(3) * 0.8 ** 3,
    "ellipsoid-pushed-aslant-through-plate": ellipsoid(2.5, 2.2, 1.2, (-3, 5, 12)),
    "torus-slid-through-rod": pi * 25 * 20 - 20 * lens(5, 1.8, 5),
}

# the closed forms above, checked at the case's own numbers against its header
SELF = {
    "ring-slid-flat-past-its-hole": ring_slid(6, 2, 15),
    "ellipsoid-pushed-aslant-through-plate": ellipsoid(3, 2, 1.5, (4, 3, 12)),
    "torus-slid-through-rod": pi * 25 * 20 - 20 * lens(5, 2, 5.5),
}


def build(cid):
    with tempfile.TemporaryDirectory() as w:
        out = os.path.join(w, "o.step")
        try:
            p = subprocess.run([RUN, cid, os.path.join(w, "o.stl"), out], capture_output=True,
                               text=True, timeout=120)
        except subprocess.TimeoutExpired:
            return "timeout", None
        if not os.path.exists(out):
            return "no STEP: " + (p.stdout.strip().splitlines() or ["?"])[-1][:150], None
        verdict = step.verdict(step.read(out))
        return verdict, mass.read(out)


def rel(a, b):
    return a / b - 1


def main():
    want = {c["id"]: c["expect"] for c in cases("swept/")}
    for name, v in SELF.items():
        print(f"formula check {name}: {rel(v, want['swept/' + name]['volume']['value']):+.1e}")
    print(f"{'variant':60} {'file':8} {'volume':>10} {'area':>10}  verdict")
    vid, m = build("mock/placement-check")
    print(f"{'mock/placement-check':60} {vid:8} {m.get('volume', 0):10.4f}")
    names = sorted(f[:-3].replace("_", "-") for f in os.listdir(os.path.join(ROOT, "kernels/cadrs/src/cases/mock")))
    for n in names:
        if n == "placement-check":
            continue
        base = n.rsplit("-", 1)[0]
        kind = n.rsplit("-", 1)[1]
        for cid in ("swept/" + base, "mock/" + n):
            verdict, m = build(cid)
            if m is None or "volume" not in m:
                print(f"{cid:60} {verdict[:60]:8} {m}")
                continue
            if cid.startswith("swept/") or kind == "placed":
                e = want["swept/" + base]
                dv, da = rel(m["volume"], e["volume"]["value"]), rel(m["area"], e["area"]["value"])
            else:
                dv, da = rel(m["volume"], RESIZED[base]), float("nan")
            ok = abs(dv) <= 1e-6 and (math.isnan(da) or abs(da) <= 1e-6)
            print(f"{cid:60} {verdict:8} {dv:+10.1e} {da:+10.1e}  {'pass' if ok else 'WRONG'}", flush=True)


if __name__ == "__main__":
    main()
