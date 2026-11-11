"""A plate with a sliver of side left between two rounds. A plate from the origin to (20, 10, 2).
Round the two edges of its face y = 10 that run along x, the one at z = 0 and the one at z = 2,
each to radius 0.9999, leaving a strip of the face 0.0002 tall between the rounds."""
import usd_brep as sm

LENGTH, WIDTH, THICKNESS = 20, 10, 2
RADIUS = 0.9999
TOLERANCE = 1e-7


def middle(edge):
    """The point halfway along an edge, by its parameter."""
    low, high = edge.parameter_range()
    return edge.point_at((low + high) / 2)


def build():
    plate = sm.create_box((0, 0, 0), LENGTH, WIDTH, THICKNESS)
    # The edges of the face y = 10 along x are the ones through (10, 10, 0) and (10, 10, 2).
    edges = [edge for edge in plate.edges()
             if abs(middle(edge)[0] - LENGTH / 2) < TOLERANCE
             and abs(middle(edge)[1] - WIDTH) < TOLERANCE]
    assert len(edges) == 2, f"{len(edges)} edges along x at y = {WIDTH}"
    return sm.fillet_edges(plate, edges, radius=RADIUS)
