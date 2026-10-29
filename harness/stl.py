"""Reading an STL, and what can be checked of a solid from its triangles alone.

The harness does not trust an adapter's own word that its solid is valid: it reads the triangles
the adapter wrote and checks them independently.
"""
import struct


def read(path):
    """The triangles of an STL file (binary or ASCII), each three (x, y, z) tuples."""
    with open(path, "rb") as f:
        data = f.read()
    if len(data) >= 84:
        n = struct.unpack_from("<I", data, 80)[0]
        if len(data) == 84 + 50 * n:
            tris = []
            for k in range(n):
                v = struct.unpack_from("<12f", data, 84 + 50 * k)
                tris.append((tuple(v[3:6]), tuple(v[6:9]), tuple(v[9:12])))
            return tris
    text = data.decode("ascii", errors="replace").split()
    verts = []
    for i, word in enumerate(text):
        if word == "vertex":
            verts.append(tuple(float(x) for x in text[i + 1:i + 4]))
    if len(verts) % 3:
        raise ValueError("an ASCII STL whose vertices are not in threes")
    return [tuple(verts[i:i + 3]) for i in range(0, len(verts), 3)]


def check(tris):
    """What the triangles say of the solid they bound.

    Vertices are joined where their coordinates are equal. The surface is *closed* when every edge
    is used exactly twice, once each way (so it is also consistently oriented and manifold along its
    edges), and its *volume* is the divergence theorem's: positive when the normals face out.
    """
    index = {}
    def vid(p):
        return index.setdefault(p, len(index))
    uses = {}
    volume = 0.0
    degenerate = 0
    for a, b, c in tris:
        ia, ib, ic = vid(a), vid(b), vid(c)
        if ia == ib or ib == ic or ic == ia:
            degenerate += 1
            continue
        for e in ((ia, ib), (ib, ic), (ic, ia)):
            uses[e] = uses.get(e, 0) + 1
        volume += (a[0] * (b[1] * c[2] - b[2] * c[1])
                   - a[1] * (b[0] * c[2] - b[2] * c[0])
                   + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0
    bad = 0
    for (i, j), n in uses.items():
        if n != 1 or uses.get((j, i), 0) != 1:
            bad += 1
    return {
        "triangles": len(tris),
        "degenerate": degenerate,
        "unmatched_edges": bad,
        "closed": bad == 0 and len(tris) > 0,
        "volume": volume,
    }
