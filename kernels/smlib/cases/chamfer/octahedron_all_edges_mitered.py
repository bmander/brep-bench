"""An octahedron with every edge chamfered, its corners mitred. The octahedron
|x| + |y| + |z| ≤ 10, its six corners on the axes, 10 from the origin. Chamfer all twelve of its
edges, each with the same distance √6 on both of its faces, measured across the face square to
the edge: the chamfer of the edge from (10, 0, 0) to (0, 10, 0) is the plane x + y = 8. No face
closes a corner: the four chamfers at each corner meet in a point. It is what chamfering the edges
one after another leaves, each edge its own chamfer, ending on the faces at its ends, the chamfers
made before it among them."""
import itertools
import math

import usd_brep as sm

REACH = 10   # each corner's distance from the origin
DISTANCE = math.sqrt(6)
# The flat section joins the lines where a round would touch the two faces, and the kernel takes
# the round's radius: those lines lie at the radius over the tangent of half the angle between
# the faces from the edge. An octahedron's faces meet at arccos(−1/3).
RADIUS = DISTANCE * math.tan(math.acos(-1 / 3) / 2)
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane
TOLERANCE = 1e-7


def corner_pairs():
    """The ends of each of the octahedron's twelve edges: two corners on different axes."""
    corners = [tuple(sign * REACH if i == axis else 0 for i in range(3))
               for axis in range(3) for sign in (1, -1)]
    for a, b in itertools.combinations(corners, 2):
        if math.dist(a, b) < 2 * REACH - TOLERANCE:   # not opposite corners
            yield a, b


def edge_between(solid, a, b):
    """The edge of `solid` lying on the segment from `a` to `b`: what is left of the octahedron's
    edge there, the chamfers before it having cut its ends off."""
    def on_segment(point):
        return abs(math.dist(a, point) + math.dist(point, b) - math.dist(a, b)) < TOLERANCE
    for edge in solid.edges():
        if all(on_segment(vertex.point()) for vertex in edge.vertices()):
            return edge
    raise AssertionError(f"no edge between {a} and {b}")



def octahedron():
    # Two square pyramids base to base. A pyramid's base is square to the x and y axes: its
    # corners are turned onto them.
    def pyramid():
        return sm.rotate(sm.create_pyramid((0, 0, 0), REACH * math.sqrt(2), REACH),
                         (0, 0, 0), (0, 0, 1), 45)
    solid = sm.boolean_union(pyramid(), sm.rotate(pyramid(), (0, 0, 0), (1, 0, 0), 180))
    assert solid.edge_count() == 12, f"{solid.edge_count()} edges"
    return solid


def build():
    solid = octahedron()
    for a, b in corner_pairs():   # each edge its own chamfer
        sm.fillet_edges(solid, [edge_between(solid, a, b)], radius=RADIUS, xsect_type=LINEAR)
    return solid
