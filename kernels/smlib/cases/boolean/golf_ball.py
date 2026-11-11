"""A golf ball. A ball of radius 10 centred on the origin, with 62 dimples, each cut by a ball of
radius 1.2 centred 10.6 out towards one of the 12 vertices, 30 edge midpoints and 20 face centres
of the icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0) and (±φ, 0, ±1). Each ball's poles on
the vertical through its centre; each dimple its own cut."""
from itertools import combinations
from math import dist, sqrt

import usd_brep as sm

RADIUS = 10
DIMPLE_RADIUS = 1.2
DIMPLE_CENTRE = 10.6


def unit(p):
    n = sqrt(sum(x * x for x in p))
    return tuple(x / n for x in p)


def directions():
    """The icosahedron's vertices, edges' midpoints and faces' centres, as unit vectors."""
    phi = (1 + sqrt(5)) / 2
    vertices = []
    for a in (-1, 1):
        for b in (-phi, phi):
            vertices += [(0, a, b), (a, b, 0), (b, 0, a)]
    # neighbours are an edge, 2, apart
    def edge(i, j):
        return abs(dist(vertices[i], vertices[j]) - 2) < 1e-9
    out = [unit(v) for v in vertices]
    out += [unit([x + y for x, y in zip(vertices[i], vertices[j])])
            for i, j in combinations(range(12), 2) if edge(i, j)]
    out += [unit([x + y + z for x, y, z in zip(vertices[i], vertices[j], vertices[k])])
            for i, j, k in combinations(range(12), 3) if edge(i, j) and edge(j, k) and edge(i, k)]
    return out


def build():
    # SMLib's sphere has its poles above and below its centre, on the vertical through it.
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    dimples = directions()
    assert len(dimples) == 62
    for d in dimples:
        cutter = sm.create_sphere(tuple(DIMPLE_CENTRE * x for x in d), DIMPLE_RADIUS)
        ball = sm.boolean_difference(ball, cutter)
    return ball
