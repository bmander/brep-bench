#!/usr/bin/env python3
"""Draw every case: a third-angle sheet of its expected solid (front, top and right views at one
scale, and an isometric view), and the explorer's thumbnail of it, as PNGs under
site/drawings/<case>/.

    python3 harness/draw.py              # every case
    python3 harness/draw.py boolean/     # the cases whose id starts with this
    python3 harness/draw.py a/b c/d      # the cases whose id starts with any of these

The solid drawn is a result that passed: the reference kernel's (kernels.json, `"reference":
true`) where it passed, else the first kernel's that did; a kernel whose mesh of it has no
triangles is passed over for the next. Its triangles are rasterised with a
depth buffer, lightly shaded, and outlined where the depth jumps (silhouettes) or the surface turns
sharply (creases), as a technical drawing is. The thumbnail is the isometric view again, small, lit
hard on a dark ground (GRAPHITE), so a solid stands out from its ground and its turned faces from
each other at the explorer's forty pixels.

A case is drawn again only when what its drawing is made from changed: the kernel it is drawn
from, that kernel's mesh, or this code. Each drawing's stamp (a hash of the mesh and of the code)
is kept in site/drawings/index.json, so a run that adds or changes a kernel redraws only the cases
whose drawing that changes, not every case it ran. Standard library only.
"""
import hashlib
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
# The isometric view's size.
ISO = 320
# The thumbnail's: the explorer shows it at 40 pixels, other pages at 64, on screens of up to twice
# that density.
THUMB = 160
# Rendered at this many times the size and averaged down, so lines are smooth.
SUPER = 2
# A crease is drawn where neighbouring facets turn by more than this.
CREASE = math.radians(28)
# Facets turned further from the eye than this (the cosine) draw no crease.
GRAZING = 0.2

# How a view is lit and coloured. A facet's tone runs from `dark` to `light` by
# ambient + diffuse·(n·key), key a light towards the viewer, to its left and up (its coordinates
# along the view's right, up and towards-the-viewer directions); `shine` (a weight and an exponent)
# whitens a facet near the key's mirror direction. `ground` is behind the solid, `ink` its lines.
# PAPER, the sheet's: a technical drawing, lines on white with the faces only told apart.
PAPER = {"ground": (252, 252, 250), "ink": (34, 36, 40), "light": (255, 255, 255),
         "dark": (196, 203, 214), "key": (-0.45, 0.6, 1), "ambient": 0.25, "diffuse": 0.75,
         "shine": None}
# GRAPHITE, the thumbnail's: a near-black ground, no ambient light and a high key, so a face turned
# from the light goes dark and the solid's form reads at a glance.
GRAPHITE = {"ground": (36, 36, 38), "ink": (12, 12, 14), "light": (236, 240, 246),
            "dark": (52, 60, 74), "key": (-0.7, 0.9, 0.6), "ambient": 0.0, "diffuse": 1.0,
            "shine": (0.25, 60)}

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


def render(tris, basis, scale, box, margin, look=PAPER):
    """The triangles seen along `basis` at `scale` pixels a unit, `box` (the view's extent) framed
    by `margin` pixels, lit and coloured as `look` says: an RGB image, as rows of bytes."""
    r, u, w = basis
    x0, x1, y0, y1 = box
    k = scale * SUPER
    m = margin * SUPER
    width = int(math.ceil((x1 - x0) * k)) + 2 * m
    height = int(math.ceil((y1 - y0) * k)) + 2 * m
    depth = [-math.inf] * (width * height)
    face = [-1] * (width * height)
    normals, slopes, facing = [], [], []
    kr, ku, kw = look["key"]
    light = unit(tuple(kr * r[j] + ku * u[j] + kw * w[j] for j in range(3)))
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
    lit, dark = look["light"], look["dark"]
    shine = look["shine"]
    half = unit(tuple(light[j] + w[j] for j in range(3)))
    tone = [None] * len(normals)
    for fi, n in enumerate(normals):
        s = min(1.0, look["ambient"] + look["diffuse"] * max(0.0, dot(n, light)))
        t = [dark[j] + (lit[j] - dark[j]) * s for j in range(3)]
        if shine:
            h = min(1.0, shine[0] * max(0.0, dot(n, half)) ** shine[1])
            t = [c + (255 - c) * h for c in t]
        tone[fi] = tuple(int(c) for c in t)
    ink, ground = look["ink"], look["ground"]
    big = []
    for py in range(height):
        row = py * width
        line = []
        for px in range(width):
            i = row + px
            f = face[i]
            edge = False
            for j in (i - 1, i + 1, i - width, i + width):
                if j < 0 or j >= width * height or (j == i - 1 and px == 0) or (
                        j == i + 1 and px == width - 1):
                    continue
                g = face[j]
                if f < 0:
                    continue
                if g < 0:
                    edge = True
                    break
                if g != f:
                    # a silhouette over a farther surface: a jump more than this facet's own
                    # slope makes from one pixel to the next
                    if depth[i] - depth[j] > jump + slopes[f]:
                        edge = True
                        break
                    # a crease, between facets both turned towards the eye (near a silhouette
                    # a smooth surface's facets crowd into one pixel at steep turns)
                    if (facing[f] > GRAZING and facing[g] > GRAZING
                            and dot(normals[f], normals[g]) < cos_crease):
                        edge = True
                        break
            line.append(ink if edge else (tone[f] if f >= 0 else ground))
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
    """The three views at one scale, the isometric view and the thumbnail, written into
    `out_dir`."""
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
    # square, the solid centred, so its ground fills the tile it is shown in
    thumb_margin = 8
    side = max(box[1] - box[0], box[3] - box[2], 1e-9)
    cx, cy = (box[0] + box[1]) / 2, (box[2] + box[3]) / 2
    square = (cx - side / 2, cx + side / 2, cy - side / 2, cy + side / 2)
    thumb_scale = (THUMB - 2 * thumb_margin) / side
    rows = render(tris, iso, thumb_scale, square, thumb_margin, GRAPHITE)
    with open(os.path.join(out_dir, "thumb.png"), "wb") as f:
        f.write(png(rows))
    sizes["thumb"] = (len(rows[0]) // 3, len(rows))
    return sizes


def has_triangles(path):
    """Whether an STL file (binary or ASCII, as stl.read reads them) holds a triangle at all."""
    with open(path, "rb") as f:
        data = f.read()
    if len(data) >= 84:
        n = struct.unpack_from("<I", data, 80)[0]
        if len(data) == 84 + 50 * n:
            return n > 0
    return b"vertex" in data


def drawn_from(case_id, results, order):
    """The kernel whose result is drawn for a case: the first in `order` that passed it with a
    mesh of some triangles. A mesh of none (a solid Open CASCADE could not mesh) is passed over,
    and said so, for the next kernel's."""
    for name in order:
        row = results.get(name, {}).get(case_id)
        mesh = os.path.join(ROOT, "results", "meshes", name, case_id + ".stl")
        if row and row["outcome"] in ("pass", "built") and os.path.exists(mesh):
            if has_triangles(mesh):
                return name, mesh
            print(f"{case_id}: {name}'s mesh has no triangles, not drawn from", flush=True)
    return None, None


def stamp(mesh):
    """What a drawing of `mesh` is made from: a hash of the mesh and of the code that draws it."""
    h = hashlib.sha256()
    for path in (os.path.abspath(__file__), stl.__file__, mesh):
        with open(path, "rb") as f:
            h.update(hashlib.sha256(f.read()).digest())
    return h.hexdigest()


def current(drawn, name, made_from, out_dir):
    """Whether the last drawing `drawn` of a case is still the one to show."""
    return (drawn is not None and drawn.get("from") == name and drawn.get("stamp") == made_from
            and all(os.path.exists(os.path.join(out_dir, view + ".png"))
                    for view in (*VIEWS, "iso", "thumb")))


def main():
    prefixes = tuple(sys.argv[1:]) or ("",)
    with open(os.path.join(ROOT, "kernels.json")) as f:
        adapters = json.load(f)["kernels"]
    results = {}
    for a in adapters:
        path = os.path.join(ROOT, "results", a["name"] + ".json")
        if os.path.exists(path):
            with open(path) as f:
                results[a["name"]] = {c["case"]: c for c in json.load(f)["cases"]}
    # the reference first, then the rest by how many cases they pass, each once
    order = list(dict.fromkeys([a["name"] for a in adapters if a.get("reference")] +
                               sorted((n for n in results), key=lambda n: -sum(
                                   c["outcome"] == "pass" for c in results[n].values()))))
    path = os.path.join(ROOT, "site", "drawings", "index.json")
    last = {}
    if os.path.exists(path):
        with open(path) as f:
            last = json.load(f)
    index = {}
    undrawn = set()
    kept = 0
    case_ids = sorted({cid for r in results.values() for cid in r})
    for cid in case_ids:
        if not cid.startswith(prefixes):
            continue
        name, mesh = drawn_from(cid, results, order)
        if not mesh:
            print(f"{cid}: no passing result with a mesh to draw")
            undrawn.add(cid)
            continue
        out_dir = os.path.join(ROOT, "site", "drawings", cid)
        made_from = stamp(mesh)
        if current(last.get(cid), name, made_from, out_dir):
            index[cid] = last[cid]
            kept += 1
            continue
        sizes = sheet(stl.read(mesh), out_dir)
        index[cid] = {"from": name, "sizes": sizes, "stamp": made_from}
        print(f"{cid}: drawn from {name}", flush=True)
    if kept:
        print(f"{kept} drawings unchanged, kept")
    if prefixes != ("",):
        # the others' drawings are kept, but not of a case with nothing to draw any more
        index = {cid: d for cid, d in {**last, **index}.items()
                 if cid in case_ids and cid not in undrawn}
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(index, f, indent=1)


if __name__ == "__main__":
    main()
