"""A notched cube with its inside corner rounded. A cube from the origin to (20, 20, 20) with a
notch cut from one corner: the cube from (10, 10, 10) to (20, 20, 20) removed. The notch's three
inside faces meet in three concave edges, which all start at the inside corner (10, 10, 10).
Round the three to radius 3, all at once: three concave quarter cylinders, and at the corner an
eighth of a sphere."""
import usd_brep as sm

SIDE = 20
NOTCH = 10
RADIUS = 3
TOLERANCE = 1e-7


def at_inside_corner(point):
    return all(abs(c - (SIDE - NOTCH)) < TOLERANCE for c in point)


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    corner = SIDE - NOTCH
    notch = sm.create_box((corner, corner, corner), NOTCH, NOTCH, NOTCH)
    cube = sm.boolean_difference(cube, notch)
    # The three concave edges are the ones that end at the inside corner.
    edges = [edge for edge in cube.edges()
             if any(at_inside_corner(vertex.point()) for vertex in edge.vertices())]
    assert len(edges) == 3, f"{len(edges)} edges at the inside corner"
    return sm.fillet_edges(cube, edges, radius=RADIUS)
