#!/usr/bin/env python3
"""Write the case files (cases/<category>/<name>.json) from the definitions below.

Every expected value here is a closed form, written beside the number as `source`, so a reader
can check the answer without trusting any kernel.  Run it after editing a case:

    python3 cases/generate.py
"""
import json
import math
import os

PI = math.pi
HERE = os.path.dirname(os.path.abspath(__file__))
CASES = []


def case(cid, tags, ops, result, volume, area, note=""):
    """One case: `ops` builds the solid named `result`; `volume` and `area` are (value, source)."""
    CASES.append({
        "id": cid,
        "tags": tags,
        "note": note,
        "ops": ops,
        "result": result,
        "expect": {
            "volume": {"value": volume[0], "source": volume[1]},
            "area": {"value": area[0], "source": area[1]},
            "valid": True,
        },
    })


def box(id, lo, hi):
    return {"op": "box", "id": id, "min": lo, "max": hi}


def cyl(id, base, axis, r, h):
    return {"op": "cylinder", "id": id, "base": base, "axis": axis, "radius": r, "height": h}


def sphere(id, c, r):
    return {"op": "sphere", "id": id, "center": c, "radius": r}


def boolean(op, id, a, b):
    return {"op": op, "id": id, "a": a, "b": b}


# -- primitives ------------------------------------------------------------------------------

case("primitives/box", ["primitive"],
     [box("out", [0, 0, 0], [10, 20, 30])], "out",
     (6000, "10·20·30"), (2200, "2(10·20 + 20·30 + 10·30)"))

case("primitives/cylinder", ["primitive"],
     [cyl("out", [0, 0, 0], [0, 0, 1], 5, 10)], "out",
     (250 * PI, "πr²h, r=5 h=10"), (150 * PI, "2πrh + 2πr²"))

s = math.sqrt(6 ** 2 + 3 ** 2)
case("primitives/cone-frustum", ["primitive"],
     [{"op": "cone", "id": "out", "base": [0, 0, 0], "axis": [0, 0, 1],
       "radius1": 5, "radius2": 2, "height": 6}], "out",
     (78 * PI, "πh(r1² + r1r2 + r2²)/3, r1=5 r2=2 h=6"),
     (7 * PI * s + 29 * PI, "π(r1+r2)√(h²+(r1−r2)²) + π(r1² + r2²)"))

case("primitives/sphere", ["primitive"],
     [sphere("out", [0, 0, 0], 5)], "out",
     (500 * PI / 3, "4πr³/3, r=5"), (100 * PI, "4πr²"))

case("primitives/torus", ["primitive"],
     [{"op": "torus", "id": "out", "center": [0, 0, 0], "axis": [0, 0, 1],
       "major": 10, "minor": 2}], "out",
     (80 * PI ** 2, "2π²Rr², R=10 r=2"), (80 * PI ** 2, "4π²Rr"))

case("primitives/extruded-l", ["primitive", "extrude"],
     [{"op": "extrude", "id": "out",
       "polygon": [[0, 0, 0], [20, 0, 0], [20, 5, 0], [5, 5, 0], [5, 15, 0], [0, 15, 0]],
       "vector": [0, 0, 8]}], "out",
     (1200, "area 150 × 8"), (860, "2·150 + perimeter 70 × 8"))

case("primitives/revolved-sector", ["primitive", "revolve"],
     [{"op": "revolve", "id": "out",
       "polygon": [[6, 0, 0], [10, 0, 0], [10, 0, 4], [6, 0, 4]],
       "axis_origin": [0, 0, 0], "axis": [0, 0, 1], "angle": 90}], "out",
     (64 * PI, "(θ/2)(ro² − ri²)h, θ=π/2 ro=10 ri=6 h=4"),
     (64 * PI + 32, "2(θ/2)(ro²−ri²) + θ(ro+ri)h + 2(ro−ri)h"))

case("primitives/revolved-washer", ["primitive", "revolve"],
     [{"op": "revolve", "id": "out",
       "polygon": [[6, 0, 0], [10, 0, 0], [10, 0, 4], [6, 0, 4]],
       "axis_origin": [0, 0, 0], "axis": [0, 0, 1], "angle": 360}], "out",
     (256 * PI, "π(ro² − ri²)h"), (256 * PI, "2π(ro²−ri²) + 2π(ro+ri)h"))

# -- Booleans --------------------------------------------------------------------------------

case("boolean/box-minus-through-cylinder", ["boolean", "cut", "analytic"],
     [box("a", [0, 0, 0], [10, 10, 10]), cyl("b", [5, 5, -1], [0, 0, 1], 3, 12),
      boolean("cut", "out", "a", "b")], "out",
     (1000 - 90 * PI, "10³ − π·3²·10"), (600 + 42 * PI, "600 − 2·9π + 2π·3·10"))

case("boolean/boxes-union", ["boolean", "union", "planar"],
     [box("a", [0, 0, 0], [10, 10, 10]), box("b", [5, 5, 0], [15, 15, 10]),
      boolean("union", "out", "a", "b")], "out",
     (1750, "2·1000 − 5·5·10"), (950, "2·175 + perimeter 60 × 10"),
     note="The two boxes share their top and bottom planes.")

case("boolean/boxes-common", ["boolean", "common", "planar"],
     [box("a", [0, 0, 0], [10, 10, 10]), box("b", [5, 5, 0], [15, 15, 10]),
      boolean("common", "out", "a", "b")], "out",
     (250, "5·5·10"), (250, "2·25 + 20·10"))

case("boolean/boxes-cut", ["boolean", "cut", "planar"],
     [box("a", [0, 0, 0], [10, 10, 10]), box("b", [5, 5, 0], [15, 15, 10]),
      boolean("cut", "out", "a", "b")], "out",
     (750, "1000 − 250"), (550, "2·75 + perimeter 40 × 10"))

case("boolean/boxes-sharing-a-face", ["boolean", "union", "planar", "coincident"],
     [box("a", [0, 0, 0], [10, 10, 10]), box("b", [10, 0, 0], [20, 10, 10]),
      boolean("union", "out", "a", "b")], "out",
     (2000, "20·10·10"), (1000, "2(20·10 + 20·10 + 10·10)"),
     note="The boxes touch along one whole face, which must vanish.")

case("boolean/boxes-sharing-part-of-a-face", ["boolean", "union", "planar", "coincident"],
     [box("a", [0, 0, 0], [10, 10, 10]), box("b", [5, 0, 10], [15, 10, 20]),
      boolean("union", "out", "a", "b")], "out",
     (2000, "1000 + 1000"), (1100, "600 + 600 − 2·5·10"))

case("boolean/tube", ["boolean", "cut", "analytic"],
     [cyl("a", [0, 0, 0], [0, 0, 1], 5, 10), cyl("b", [0, 0, -1], [0, 0, 1], 3, 12),
      boolean("cut", "out", "a", "b")], "out",
     (160 * PI, "π(5² − 3²)·10"), (192 * PI, "2π(5+3)·10 + 2π(5² − 3²)"))

case("boolean/tube-flush-ends", ["boolean", "cut", "analytic", "coincident"],
     [cyl("a", [0, 0, 0], [0, 0, 1], 5, 10), cyl("b", [0, 0, 0], [0, 0, 1], 3, 10),
      boolean("cut", "out", "a", "b")], "out",
     (160 * PI, "π(5² − 3²)·10"), (192 * PI, "2π(5+3)·10 + 2π(5² − 3²)"),
     note="The tool's end faces lie in the target's.")

case("boolean/stepped-shaft", ["boolean", "union", "analytic", "coincident"],
     [cyl("a", [0, 0, 0], [0, 0, 1], 5, 10), cyl("b", [0, 0, 10], [0, 0, 1], 3, 10),
      boolean("union", "out", "a", "b")], "out",
     (340 * PI, "π·25·10 + π·9·10"), (210 * PI, "25π + 16π + 9π + 2π·5·10 + 2π·3·10"))

case("boolean/hemisphere", ["boolean", "cut", "analytic"],
     [sphere("a", [0, 0, 0], 5), box("b", [-10, -10, -10], [10, 10, 0]),
      boolean("cut", "out", "a", "b")], "out",
     (250 * PI / 3, "2πr³/3"), (75 * PI, "2πr² + πr²"),
     note="The cutting plane passes through the sphere's centre.")

case("boolean/sphere-pocket", ["boolean", "cut", "analytic"],
     [box("a", [-10, -10, -10], [10, 10, 0]), sphere("b", [0, 0, 0], 5),
      boolean("cut", "out", "a", "b")], "out",
     (4000 - 250 * PI / 3, "20·20·10 − 2πr³/3"), (1600 + 25 * PI, "1600 − πr² + 2πr²"))

case("boolean/napkin-ring", ["boolean", "cut", "analytic"],
     [sphere("a", [0, 0, 0], 5), cyl("b", [0, 0, -10], [0, 0, 1], 3, 20),
      boolean("cut", "out", "a", "b")], "out",
     (256 * PI / 3, "πh³/6, h = 2√(5² − 3²) = 8"), (128 * PI, "2πRh + 2πrh"))

case("boolean/steinmetz-common", ["boolean", "common", "analytic", "ssi"],
     [cyl("a", [-15, 0, 0], [1, 0, 0], 5, 30), cyl("b", [0, -15, 0], [0, 1, 0], 5, 30),
      boolean("common", "out", "a", "b")], "out",
     (2000 / 3, "16r³/3, r=5"), (400, "16r²"),
     note="Two equal cylinders crossing at right angles meet in two ellipses through two points.")

case("boolean/steinmetz-union", ["boolean", "union", "analytic", "ssi"],
     [cyl("a", [-15, 0, 0], [1, 0, 0], 5, 30), cyl("b", [0, -15, 0], [0, 1, 0], 5, 30),
      boolean("union", "out", "a", "b")], "out",
     (1500 * PI - 2000 / 3, "2πr²L − 16r³/3, r=5 L=30"),
     (700 * PI - 400, "2(2πrL + 2πr²) − 16r²"))

lens_v = PI * (4 * 5 + 6) * (2 * 5 - 6) ** 2 / 12
case("boolean/sphere-lens", ["boolean", "common", "analytic"],
     [sphere("a", [0, 0, 0], 5), sphere("b", [6, 0, 0], 5),
      boolean("common", "out", "a", "b")], "out",
     (lens_v, "π(4r + d)(2r − d)²/12, r=5 d=6"), (40 * PI, "2·2πr(r − d/2)"))

case("boolean/spheres-union", ["boolean", "union", "analytic"],
     [sphere("a", [0, 0, 0], 5), sphere("b", [6, 0, 0], 5),
      boolean("union", "out", "a", "b")], "out",
     (1000 * PI / 3 - lens_v, "2·4πr³/3 − lens"), (160 * PI, "2·4πr² − 2·2πr(r − d/2)"))

case("boolean/half-torus", ["boolean", "cut", "analytic"],
     [{"op": "torus", "id": "a", "center": [0, 0, 0], "axis": [0, 0, 1], "major": 10, "minor": 2},
      box("b", [-20, -20, -5], [0, 20, 5]),
      boolean("cut", "out", "a", "b")], "out",
     (40 * PI ** 2, "π²Rr²"), (40 * PI ** 2 + 8 * PI, "2π²Rr + 2πr²"))

case("boolean/half-cylinder-boss-tangent", ["boolean", "union", "analytic", "tangent"],
     [box("a", [0, 0, 0], [10, 10, 10]), cyl("b", [10, 5, 0], [0, 0, 1], 5, 10),
      boolean("union", "out", "a", "b")], "out",
     (1000 + 125 * PI, "1000 + πr²h/2, r=5 h=10"), (500 + 75 * PI, "2(100 + πr²/2) + 300 + πrh"),
     note="The cylinder is tangent to two of the box's faces along two of its edges.")

drill = [box("plate", [0, 0, 0], [100, 20, 10])]
prev = "plate"
for k in range(1, 10):
    drill.append(cyl(f"hole{k}", [10 * k, 10, -1], [0, 0, 1], 2, 12))
    drill.append(boolean("cut", f"drilled{k}", prev, f"hole{k}"))
    prev = f"drilled{k}"
case("boolean/drilled-plate", ["boolean", "cut", "analytic", "sequence"],
     drill, prev,
     (20000 - 360 * PI, "100·20·10 − 9·π·2²·10"),
     (6400 + 288 * PI, "6400 − 9·2·π·2² + 9·2π·2·10"),
     note="Nine holes cut one after another.")

# -- sweeps ----------------------------------------------------------------------------------

turns, R, pitch, r = 3, 10, 5, 1
L = turns * math.sqrt((2 * PI * R) ** 2 + pitch ** 2)
helix = {"center": [0, 0, 0], "axis": [0, 0, 1], "radius": R, "pitch": pitch, "turns": turns}
case("sweep/helical-pipe", ["sweep", "helix"],
     [{"op": "sweep_circle", "id": "out", "radius": r, "helix": helix, "ends": "flat"}], "out",
     (PI * r * r * L, "πr²L, L = n√((2πR)² + p²)"), (2 * PI * r * L + 2 * PI * r * r, "2πrL + 2πr²"),
     note="A disc square to the path, swept along a helix of radius 10, pitch 5, three turns.")

case("sweep/helical-pipe-round-ends", ["sweep", "helix"],
     [{"op": "sweep_circle", "id": "out", "radius": r, "helix": helix, "ends": "round"}], "out",
     (PI * r * r * L + 4 * PI * r ** 3 / 3, "πr²L + 4πr³/3"),
     (2 * PI * r * L + 4 * PI * r * r, "2πrL + 4πr²"),
     note="The path a ball's centre runs: the pipe and a half ball at each end.")

case("sweep/circle-along-arc", ["sweep", "planar-path"],
     [{"op": "sweep_circle", "id": "out", "radius": 2, "ends": "flat",
       "helix": {"center": [0, 0, 0], "axis": [0, 0, 1], "radius": 10, "pitch": 0,
                 "turns": 0.25}}], "out",
     (20 * PI ** 2, "πr²·(πR/2), r=2 R=10"), (20 * PI ** 2 + 8 * PI, "2πr·(πR/2) + 2πr²"))

# -- fillets ---------------------------------------------------------------------------------

case("fillet/box-one-edge", ["fillet"],
     [box("a", [0, 0, 0], [10, 10, 10]),
      {"op": "fillet", "id": "out", "target": "a", "radius": 2,
       "edges": {"nearest": [[10, 10, 5]]}}], "out",
     (960 + 10 * PI, "1000 − r²(1 − π/4)·10, r=2"),
     (552 + 12 * PI, "600 − 2·r·10 + (π/2)r·10 − 2r²(1 − π/4)"))

case("fillet/box-all-edges", ["fillet", "corner"],
     [box("a", [0, 0, 0], [10, 10, 10]),
      {"op": "fillet", "id": "out", "target": "a", "radius": 1, "edges": {"all": True}}], "out",
     (896 + 24 * PI + 4 * PI / 3, "8³ + 6·8²·r + (π/4)·96·r² + 4πr³/3, r=1"),
     (384 + 52 * PI, "6·8² + (π/2)·96·r + 4πr²"),
     note="Three fillets meet at each corner.")

d = (10 - 3 * PI) / (12 - 3 * PI)
case("fillet/cylinder-rim", ["fillet"],
     [cyl("a", [0, 0, 0], [0, 0, 1], 5, 10),
      {"op": "fillet", "id": "out", "target": "a", "radius": 1,
       "edges": {"nearest": [[-0.9, 5.12, 10.2]]}}], "out",
     (250 * PI - 2 * PI * (5 - d) * (1 - PI / 4),
      "πR²h − 2π(R − d)(1 − π/4)r², d = r(10 − 3π)/(12 − 3π) (Pappus), R=5 h=10 r=1"),
     (133 * PI + 4 * PI ** 2, "150π − 9π − 10π + 2π(4 + 2/π)(π/2) (Pappus)"))


def main():
    for c in CASES:
        path = os.path.join(HERE, c["id"] + ".json")
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            json.dump(c, f, indent=2)
            f.write("\n")
    print(f"{len(CASES)} cases written")


if __name__ == "__main__":
    main()
