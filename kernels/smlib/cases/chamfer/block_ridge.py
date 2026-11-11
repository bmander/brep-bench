"""A block whose top is chamfered into a ridge. A block from the origin to (10, 4, 6). Chamfer the
two edges of its top face z = 6 that run along x, the one at y = 0 and the one at y = 4, each with
the same distance 2 on both of its faces, half the top face's width, so that the top face
vanishes and the chamfers meet along the ridge y = 2, z = 6."""
import usd_brep as sm

LENGTH, WIDTH, HEIGHT = 10, 4, 6
DISTANCE = 2
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane
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
    # A flat cut between the lines where a round of that radius would touch the faces: at a
    # square edge, the lines at the radius from it.
    return sm.fillet_edges(block, edges, radius=DISTANCE, xsect_type=LINEAR)
