"""Reading an STL, and what can be checked of a solid from its triangles alone.

The harness does not trust a runner's own word that its solid is valid: it reads the triangles
the runner wrote and checks them independently.
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


def about_centroid(volume, m1, m2, origin):
    """The centre of gravity and the inertia tensor about it, of unit density, from a solid's
    volume and its first moments (∫x, ∫y, ∫z) and second (∫x_i x_j, 3×3) about `origin`:
    (centroid, inertia), as lists."""
    g = [m1[i] / volume for i in range(3)]
    s2 = [[m2[i][j] - volume * g[i] * g[j] for j in range(3)] for i in range(3)]
    tr = s2[0][0] + s2[1][1] + s2[2][2]
    inertia = [[(tr - s2[i][i]) if i == j else -s2[i][j] for j in range(3)] for i in range(3)]
    return [origin[i] + g[i] for i in range(3)], inertia


def check(tris):
    """What the triangles say of the solid they bound.

    Vertices are joined where their coordinates are equal. The surface is *closed* when every edge
    is used exactly twice, once each way (so it is also consistently oriented and manifold along its
    edges), its *volume* is the divergence theorem's (positive when the normals face out), and its
    *area* is its triangles' summed. Its *centroid* and its *inertia* (of unit density, about the
    centroid) are those of the polyhedron the triangles bound, summed over the tetrahedra each makes
    with a point; as harness/mass.py gives them of a STEP file's solid.
    """
    index = {}
    def vid(p):
        return index.setdefault(p, len(index))
    uses = {}
    volume = 0.0
    area = 0.0
    degenerate = 0
    # the tetrahedra's apex: the first vertex, so that the sums do not cancel far from the origin
    o = tris[0][0] if tris else (0.0, 0.0, 0.0)
    m1 = [0.0, 0.0, 0.0]
    xx = yy = zz = xy = yz = zx = 0.0
    for a, b, c in tris:
        ia, ib, ic = vid(a), vid(b), vid(c)
        if ia == ib or ib == ic or ic == ia:
            degenerate += 1
            continue
        for e in ((ia, ib), (ib, ic), (ic, ia)):
            uses[e] = uses.get(e, 0) + 1
        u = (b[0] - a[0], b[1] - a[1], b[2] - a[2])
        v = (c[0] - a[0], c[1] - a[1], c[2] - a[2])
        n = (u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0])
        area += 0.5 * (n[0] ** 2 + n[1] ** 2 + n[2] ** 2) ** 0.5
        # the tetrahedron with the apex o: its volume, and its moments about o
        px, py, pz = a[0] - o[0], a[1] - o[1], a[2] - o[2]
        qx, qy, qz = b[0] - o[0], b[1] - o[1], b[2] - o[2]
        rx, ry, rz = c[0] - o[0], c[1] - o[1], c[2] - o[2]
        t = (px * (qy * rz - qz * ry) - py * (qx * rz - qz * rx) + pz * (qx * ry - qy * rx)) / 6
        volume += t
        sx, sy, sz = px + qx + rx, py + qy + ry, pz + qz + rz
        m1[0] += t * sx / 4
        m1[1] += t * sy / 4
        m1[2] += t * sz / 4
        w = t / 20
        xx += w * (px * px + qx * qx + rx * rx + sx * sx)
        yy += w * (py * py + qy * qy + ry * ry + sy * sy)
        zz += w * (pz * pz + qz * qz + rz * rz + sz * sz)
        xy += w * (px * py + qx * qy + rx * ry + sx * sy)
        yz += w * (py * pz + qy * qz + ry * rz + sy * sz)
        zx += w * (pz * px + qz * qx + rz * rx + sz * sx)
    bad = 0
    for (i, j), n in uses.items():
        if n != 1 or uses.get((j, i), 0) != 1:
            bad += 1
    centroid = inertia = None
    if volume:
        centroid, inertia = about_centroid(volume, m1, [[xx, xy, zx], [xy, yy, yz], [zx, yz, zz]], o)
    return {
        "triangles": len(tris),
        "centroid": centroid,
        "inertia": inertia,
        "degenerate": degenerate,
        "unmatched_edges": bad,
        "closed": bad == 0 and len(tris) > 0,
        "volume": volume,
        "area": area,
    }
