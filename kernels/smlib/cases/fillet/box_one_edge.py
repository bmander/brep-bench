"""A cube with one edge rounded. A cube of side 10 between the origin and (10, 10, 10). Round the
edge where its faces x = 10 and y = 10 meet, to radius 2."""
import usd_brep as sm

SIDE = 10
RADIUS = 2
TOLERANCE = 1e-7


def on_both_faces(point):
    """Whether a point lies on the face x = 10 and on the face y = 10."""
    return abs(point[0] - SIDE) < TOLERANCE and abs(point[1] - SIDE) < TOLERANCE


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    # The edge where the faces x = 10 and y = 10 meet is the one whose two ends lie on both.
    edges = [edge for edge in cube.edges()
             if all(on_both_faces(vertex.point()) for vertex in edge.vertices())]
    assert len(edges) == 1, f"{len(edges)} edges at x = y = {SIDE}"
    return sm.fillet_edges(cube, edges, radius=RADIUS)   # rounds the cube in place and returns it
