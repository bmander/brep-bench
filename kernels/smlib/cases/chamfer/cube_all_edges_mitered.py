"""A cube with every edge chamfered, its corners mitred. A cube of side 10 between the origin and
(10, 10, 10). Chamfer all twelve of its edges, each with the same distance 2 on both of its
faces. No face closes a corner: each chamfer runs on until it meets the others, so the three at
each corner meet in a point. It is what chamfering the edges one after another leaves, each edge
its own chamfer, ending on the faces at its ends, the chamfers made before it among them."""
import usd_brep as sm

SIDE = 10
DISTANCE = 2
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def middles():
    """The middle of each of the cube's twelve edges: one coordinate half the side, the other two
    each at 0 or at the side."""
    for axis in range(3):
        for u in (0, SIDE):
            for v in (0, SIDE):
                point = [u, v]
                point.insert(axis, SIDE / 2)
                yield point


def edge_through(solid, point):
    """The edge of `solid` whose middle is `point`. What is left of a cube's edge keeps its
    middle, each chamfer before it cutting as much off either end."""
    for edge in solid.edges():
        if max(abs(a - b) for a, b in zip(middle(edge), point)) < TOLERANCE:
            return edge
    raise AssertionError(f"no edge through {point}")


def build():
    # Each edge its own chamfer: a flat cut between the lines where a round of that radius would
    # touch the edge's faces, which at a square edge are at the radius from it.
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    for point in middles():
        sm.fillet_edges(cube, [edge_through(cube, point)], radius=DISTANCE, xsect_type=LINEAR)
    return cube
