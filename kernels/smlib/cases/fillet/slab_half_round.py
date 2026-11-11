"""A block whose top is rounded into a half-cylinder. A block from the origin to (10, 4, 6). Round
the two edges of its top face z = 6 that run along x, the one at y = 0 and the one at y = 4, each
to radius 2, half the top face's width, so that the top face vanishes and the rounds meet along
the line y = 2, z = 6."""
import usd_brep as sm

LENGTH, WIDTH, HEIGHT = 10, 4, 6
RADIUS = 2
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def build():
    block = sm.create_box((0, 0, 0), LENGTH, WIDTH, HEIGHT)
    # The top face's edges along x are the ones through (5, 0, 6) and (5, 4, 6).
    edges = [edge for edge in block.edges()
             if abs(middle(edge)[0] - LENGTH / 2) < TOLERANCE
             and abs(middle(edge)[2] - HEIGHT) < TOLERANCE]
    assert len(edges) == 2, f"{len(edges)} edges along x at z = {HEIGHT}"
    return sm.fillet_edges(block, edges, radius=RADIUS)
