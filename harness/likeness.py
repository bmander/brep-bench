#!/usr/bin/env python3
"""How like a reference solid a kernel's solid is: the measure of a case that names a `reference`
(SPEC.md, "A reference shape"), shown and not graded.

    python3 harness/likeness.py CASE FILE.stl ...     # each file's likeness to CASE's reference
    python3 harness/likeness.py CASE --kernel NAME    # the triangles a run kept, and NAME's lines

The likeness is the volume the two solids hold in common over the volume either holds (their
intersection over their union, 1 for the same solid). Both are read from triangles, so a mesh
with gaps or doubled faces is read as the solid its faces wind round: each column of a square
grid, upright, crosses the faces, and each crossing adds one to the winding number below a face
that faces down and takes one away above a face that faces up; the column is inside where the
number is above nought. So the volumes are integrated exactly along each column and summed over
the grid, whose spacing is a five-hundredth of the reference's largest extent (0.12 for a solid
60 across), the grid shifted off any round coordinate. Standard library only.
"""
import gzip
import math
import os
import sys
import tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import stl  # noqa: E402
from suite import ROOT, cases, kernels, source  # noqa: E402

# the grid's spacing, as a fraction of the reference's largest extent
SPACING = 1 / 500


def header(case):
    with open(case["path"], encoding="utf-8") as f:
        return tomllib.loads(f.read().split("+++", 2)[1])


def reference(case):
    """The path of the reference solid `case` (a case of `suite.cases`) names, or None."""
    ref = header(case).get("reference")
    return os.path.join(os.path.dirname(case["path"]), ref) if ref else None


def reference_mesh(case):
    """The path of the triangles of `case`'s reference the likeness reads: its `reference_mesh`,
    or the reference itself where that is a mesh; None where it names no reference."""
    meta = header(case)
    ref = meta.get("reference_mesh", meta.get("reference"))
    return os.path.join(os.path.dirname(case["path"]), ref) if ref else None


def triangles(path):
    """An STL file's triangles, the file gzipped or not."""
    if not path.endswith(".gz"):
        return stl.read(path)
    import tempfile
    with tempfile.NamedTemporaryFile(suffix=".stl") as f:
        f.write(gzip.decompress(open(path, "rb").read()))
        f.flush()
        return stl.read(f.name)


def crossings(tris, x0, y0, h, into):
    """Each column (i, j) at (x0 + i·h, y0 + j·h) a triangle crosses: its height there and the
    change it makes to the winding number going up, added to `into[(i, j)]`."""
    for a, b, c in tris:
        d = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
        if d == 0:
            continue  # a face standing upright: no column crosses it
        step = -1 if d > 0 else 1  # up through a face facing up, out of the solid
        xs, ys = (a[0], b[0], c[0]), (a[1], b[1], c[1])
        for i in range(math.ceil((min(xs) - x0) / h), math.floor((max(xs) - x0) / h) + 1):
            px = x0 + i * h
            for j in range(math.ceil((min(ys) - y0) / h), math.floor((max(ys) - y0) / h) + 1):
                py = y0 + j * h
                u = ((b[0] - px) * (c[1] - py) - (b[1] - py) * (c[0] - px)) / d
                v = ((c[0] - px) * (a[1] - py) - (c[1] - py) * (a[0] - px)) / d
                if u >= 0 and v >= 0 and u + v <= 1:
                    z = u * a[2] + v * b[2] + (1 - u - v) * c[2]
                    into.setdefault((i, j), []).append((z, step))


def inside(events):
    """A column's runs inside the solid, from its crossings: (from, to) pairs."""
    runs, w, start = [], 0, None
    for z, step in sorted(events):
        w += step
        if w > 0 and start is None:
            start = z
        elif w <= 0 and start is not None:
            runs.append((start, z))
            start = None
    return runs


def length(runs):
    return sum(b - a for a, b in runs)


def common(r, s):
    out, i, j = 0.0, 0, 0
    while i < len(r) and j < len(s):
        out += max(0.0, min(r[i][1], s[j][1]) - max(r[i][0], s[j][0]))
        if r[i][1] < s[j][1]:
            i += 1
        else:
            j += 1
    return out


def likeness(tris, ref):
    """How like the triangles `ref` bound the triangles `tris` bound: {iou, common, union,
    volume, reference}, the volumes integrated along the grid's columns."""
    pts = [p for t in ref for p in t]
    size = max(max(p[k] for p in pts) - min(p[k] for p in pts) for k in range(3))
    h = size * SPACING
    x0 = min(p[0] for p in pts) + h * 0.3183098861837907  # (1/π: off any round coordinate)
    y0 = min(p[1] for p in pts) + h * 0.2718281828459045
    mine, theirs = {}, {}
    crossings(tris, x0, y0, h, mine)
    crossings(ref, x0, y0, h, theirs)
    vol = rvol = both = 0.0
    for col in set(mine) | set(theirs):
        r, s = inside(mine.get(col, [])), inside(theirs.get(col, []))
        a, b, c = length(r), length(s), common(r, s)
        vol, rvol, both = vol + a, rvol + b, both + c
    area = h * h
    union = vol + rvol - both
    return {"iou": both / union if union else 1.0, "common": both * area, "union": union * area,
            "volume": vol * area, "reference": rvol * area}


def main(argv):
    if len(argv) < 2:
        print(__doc__.split("\n\n")[1])
        return 2
    case = next((c for c in cases(argv[0]) if c["id"] == argv[0]), None)
    if case is None or reference_mesh(case) is None:
        print(f"{argv[0]}: no such case, or it names no reference", file=sys.stderr)
        return 2
    ref = triangles(reference_mesh(case))
    if argv[1] == "--kernel":
        from highlight import loc
        k = next(k for k in kernels() if k["name"] == argv[2])
        src = source(k, case["id"])
        lines = None
        if src and os.path.exists(src):
            with open(src, encoding="utf-8") as f:
                lines = loc(f.read(), src)
        files = [os.path.join(ROOT, "results", "meshes", k["name"], case["id"] + ".stl")]
    else:
        lines, files = None, argv[1:]
    for path in files:
        got = likeness(triangles(path), ref)
        print(f"{path}: likeness {got['iou']:.4f} (common {got['common']:.1f}, union "
              f"{got['union']:.1f}, its volume {got['volume']:.1f}, the reference's "
              f"{got['reference']:.1f})" + (f", {lines} lines" if lines is not None else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
