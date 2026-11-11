"""A rounded block with its top rim rounded. A block 20 by 12 by 5, from (−10, −6, 0) to
(10, 6, 5), with its four vertical edges rounded to radius 3. Round the rim of its top face to
radius 1: eight edges, four straight and four quarter circles, each running on tangent into the
next, the round running around all eight unbroken."""
import usd_brep as sm

HALF_LENGTH, HALF_WIDTH, HEIGHT = 10, 6, 5
CORNER_RADIUS = 3
RADIUS = 1
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def build():
    block = sm.create_box((-HALF_LENGTH, -HALF_WIDTH, 0), 2 * HALF_LENGTH, 2 * HALF_WIDTH, HEIGHT)
    # The vertical edges are the ones halfway up.
    uprights = [edge for edge in block.edges() if abs(middle(edge)[2] - HEIGHT / 2) < TOLERANCE]
    assert len(uprights) == 4, f"{len(uprights)} vertical edges"
    sm.fillet_edges(block, uprights, radius=CORNER_RADIUS)
    # The top face's rim is every edge lying at z = 5.
    rim = [edge for edge in block.edges()
           if all(abs(point[2] - HEIGHT) < TOLERANCE
                  for point in [middle(edge)] + [vertex.point() for vertex in edge.vertices()])]
    assert len(rim) == 8, f"{len(rim)} edges around the top face"
    return sm.fillet_edges(block, rim, radius=RADIUS)
