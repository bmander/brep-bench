#!/usr/bin/env python3
"""Draw every case: a third-angle sheet of its expected solid (front, top and right views at one
scale, and an isometric view), as PNGs under site/drawings/<case>/.

    python3 harness/draw.py              # every case
    python3 harness/draw.py boolean/     # the cases whose id starts with this

The solid drawn is a result that passed: the reference kernel's (kernels.json, `"reference":
true`) where it passed, else the first kernel's that did. Its triangles are rasterised with a
depth buffer, lightly shaded, and outlined where the depth jumps (silhouettes) or the surface turns
sharply (creases), as a technical drawing is. Standard library only.
"""
import json
import math
import os
import struct
import sys
import zlib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stl  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Pixels per unit of length are chosen so the sheet's views fit in this many pixels across.
SHEET = 560
# The isometric view's size, which is also the scoreboard's thumbnail (shown at half).
ISO = 320
# Rendered at this many times the size and averaged down, so lines are smooth.
SUPER = 2
# A crease is drawn where neighbouring facets turn by more than this.
CREASE = math.radians(28)
# Facets turned further from the eye than this (the cosine) draw no crease.
GRAZING = 0.2

PAPER = (252, 252, 250)
INK = (34, 36, 40)
LIGHT_TONE = (255, 255, 255)
DARK_TONE = (196, 203, 214)

# Each view: the directions to the right, up, and towards the viewer.
VIEWS = {
    "front": ((1, 0, 0), (0, 0, 1), (0, -1, 0)),
    "top": ((1, 0, 0), (0, 1, 0), (0, 0, 1)),
    "right": ((0, 1, 0), (0, 0, 1), (1, 0, 0)),
}


def unit(v):
    n = math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2])
    return (v[0] / n, v[1] / n, v[2] / n)


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def iso_view():
    """Looking from the front, right and top at once: towards (−1, 1, −1)."""
    w = unit((1, -1, 1))
    r = unit(cross((0, 0, 1), w))
    return (r, cross(w, r), w)


def bounds(tris, basis):
    """The extent of the triangles along a view's right and up directions."""
    r, u, _ = basis
    xs = [dot(p, r) for t in tris for p in t]
    ys = [dot(p, u) for t in tris for p in t]
    return min(xs), max(xs), min(ys), max(ys)


def render(tris, basis, scale, box, margin):
    """The triangles seen along `basis` at `scale` pixels a unit, `box` (the view's extent) framed
    by `margin` pixels: an RGB image, as rows of bytes."""
    r, u, w = basis
    x0, x1, y0, y1 = box
    k = scale * SUPER
    m = margin * SUPER
    width = int(math.ceil((x1 - x0) * k)) + 2 * m
    height = int(math.ceil((y1 - y0) * k)) + 2 * m
    depth = [-math.inf] * (width * height)
    face = [-1] * (width * height)
    normals, slopes, facing = [], [], []
    light = unit((-0.45 * r[0] + 0.6 * u[0] + w[0], -0.45 * r[1] + 0.6 * u[1] + w[1],
                  -0.45 * r[2] + 0.6 * u[2] + w[2]))
    for a, b, c in tris:
        n = cross((b[0] - a[0], b[1] - a[1], b[2] - a[2]), (c[0] - a[0], c[1] - a[1], c[2] - a[2]))
        if dot(n, w) <= 0:
            continue  # facing away: a closed solid's back
        n = unit(n)
        sx = [(dot(p, r) - x0) * k + m for p in (a, b, c)]
        sy = [(y1 - dot(p, u)) * k + m for p in (a, b, c)]
        sz = [dot(p, w) for p in (a, b, c)]
        area = (sx[1] - sx[0]) * (sy[2] - sy[0]) - (sx[2] - sx[0]) * (sy[1] - sy[0])
        if area == 0:
            continue
        fi = len(normals)
        normals.append(n)
        # how far the facet's depth changes from one pixel to the next
        nw = dot(n, w)
        slopes.append(max(abs(dot(n, r)), abs(dot(n, u))) / (nw * k))
        facing.append(nw)
        lo_x = max(int(min(sx)), 0)
        hi_x = min(int(max(sx)) + 1, width - 1)
        lo_y = max(int(min(sy)), 0)
        hi_y = min(int(max(sy)) + 1, height - 1)
        inv = 1.0 / area
        for py in range(lo_y, hi_y + 1):
            cy = py + 0.5
            row = py * width
            for px in range(lo_x, hi_x + 1):
                cx = px + 0.5
                l0 = ((sx[1] - cx) * (sy[2] - cy) - (sx[2] - cx) * (sy[1] - cy)) * inv
                l1 = ((sx[2] - cx) * (sy[0] - cy) - (sx[0] - cx) * (sy[2] - cy)) * inv
                l2 = 1.0 - l0 - l1
                if l0 < 0 or l1 < 0 or l2 < 0:
                    continue
                z = l0 * sz[0] + l1 * sz[1] + l2 * sz[2]
                i = row + px
                if z > depth[i]:
                    depth[i] = z
                    face[i] = fi
    # the depth a jump must exceed to be a silhouette: a few pixels' worth of length
    jump = 3.0 / k
    cos_crease = math.cos(CREASE)
    tone = [None] * len(normals)
    for fi, n in enumerate(normals):
        s = max(0.0, dot(n, light)) * 0.75 + 0.25
        tone[fi] = tuple(int(DARK_TONE[j] + (LIGHT_TONE[j] - DARK_TONE[j]) * s) for j in range(3))
    big = []
    for py in range(height):
        row = py * width
        line = []
        for px in range(width):
            i = row + px
            f = face[i]
            ink = False
            for j in (i - 1, i + 1, i - width, i + width):
                if j < 0 or j >= width * height or (j == i - 1 and px == 0) or (
                        j == i + 1 and px == width - 1):
                    continue
                g = face[j]
                if f < 0:
                    continue
                if g < 0:
                    ink = True
                    break
                if g != f:
                    # a silhouette over a farther surface: a jump more than this facet's own
                    # slope makes from one pixel to the next
                    if depth[i] - depth[j] > jump + slopes[f]:
                        ink = True
                        break
                    # a crease, between facets both turned towards the eye (near a silhouette
                    # a smooth surface's facets crowd into one pixel at steep turns)
                    if (facing[f] > GRAZING and facing[g] > GRAZING
                            and dot(normals[f], normals[g]) < cos_crease):
                        ink = True
                        break
            line.append(INK if ink else (tone[f] if f >= 0 else PAPER))
        big.append(line)
    return downsample(big, width, height)


def downsample(big, width, height):
    w, h = width // SUPER, height // SUPER
    rows = []
    n = SUPER * SUPER
    for y in range(h):
        out = bytearray()
        for x in range(w):
            acc = [0, 0, 0]
            for dy in range(SUPER):
                src = big[y * SUPER + dy]
                for dx in range(SUPER):
                    p = src[x * SUPER + dx]
                    acc[0] += p[0]
                    acc[1] += p[1]
                    acc[2] += p[2]
            out += bytes((acc[0] // n, acc[1] // n, acc[2] // n))
        rows.append(bytes(out))
    return rows


def png(rows):
    """An RGB image's rows as a PNG file."""
    h = len(rows)
    w = len(rows[0]) // 3

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data +
                struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    raw = b"".join(b"\x00" + r for r in rows)
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)) +
            chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def sheet(tris, out_dir):
    """The three views at one scale, and the isometric view, written into `out_dir`."""
    boxes = {name: bounds(tris, basis) for name, basis in VIEWS.items()}
    fx0, fx1, fy0, fy1 = boxes["front"]
    tx0, tx1, ty0, ty1 = boxes["top"]
    rx0, rx1, ry0, ry1 = boxes["right"]
    margin = 12
    # front and right side by side, top above front: fit the whole sheet in SHEET pixels
    across = (fx1 - fx0) + (rx1 - rx0)
    down = (ty1 - ty0) + (fy1 - fy0)
    scale = (SHEET - 4 * margin) / max(across, down, 1e-9)
    sizes = {}
    os.makedirs(out_dir, exist_ok=True)
    for name, basis in VIEWS.items():
        rows = render(tris, basis, scale, boxes[name], margin)
        with open(os.path.join(out_dir, name + ".png"), "wb") as f:
            f.write(png(rows))
        sizes[name] = (len(rows[0]) // 3, len(rows))
    iso = iso_view()
    box = bounds(tris, iso)
    iso_scale = (ISO - 2 * margin) / max(box[1] - box[0], box[3] - box[2], 1e-9)
    rows = render(tris, iso, iso_scale, box, margin)
    with open(os.path.join(out_dir, "iso.png"), "wb") as f:
        f.write(png(rows))
    sizes["iso"] = (len(rows[0]) // 3, len(rows))
    return sizes


def drawn_from(case_id, results, order):
    """The kernel whose result is drawn for a case: the first in `order` that passed it."""
    for name in order:
        row = results.get(name, {}).get(case_id)
        mesh = os.path.join(ROOT, "results", "meshes", name, case_id + ".stl")
        if row and row["outcome"] in ("pass", "built") and os.path.exists(mesh):
            return name, mesh
    return None, None


def main():
    prefix = sys.argv[1] if len(sys.argv) > 1 else ""
    with open(os.path.join(ROOT, "kernels.json")) as f:
        adapters = json.load(f)["kernels"]
    results = {}
    for a in adapters:
        path = os.path.join(ROOT, "results", a["name"] + ".json")
        if os.path.exists(path):
            with open(path) as f:
                results[a["name"]] = {c["case"]: c for c in json.load(f)["cases"]}
    order = ([a["name"] for a in adapters if a.get("reference")] +
             sorted((n for n in results), key=lambda n: -sum(
                 c["outcome"] == "pass" for c in results[n].values())))
    index = {}
    case_ids = sorted({cid for r in results.values() for cid in r})
    for cid in case_ids:
        if not cid.startswith(prefix):
            continue
        name, mesh = drawn_from(cid, results, order)
        if not mesh:
            print(f"{cid}: no passing result to draw")
            continue
        sizes = sheet(stl.read(mesh), os.path.join(ROOT, "site", "drawings", cid))
        index[cid] = {"from": name, "sizes": sizes}
        print(f"{cid}: drawn from {name}", flush=True)
    path = os.path.join(ROOT, "site", "drawings", "index.json")
    if prefix and os.path.exists(path):
        with open(path) as f:
            index = {**json.load(f), **index}
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(index, f, indent=1)


if __name__ == "__main__":
    main()
