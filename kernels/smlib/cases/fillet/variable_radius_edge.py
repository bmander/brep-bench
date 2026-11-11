"""A cube edge rounded with a radius growing along it. A cube of side 10 between the origin and
(10, 10, 10). Round the edge where its faces x = 10 and y = 10 meet with a radius that grows
linearly along it, r = 1 + z/10: 1 at the bottom face z = 0 and 2 at the top face z = 10."""
import usd_brep as sm

SIDE = 10
RADIUS_BOTTOM, RADIUS_TOP = 1, 2
TOLERANCE = 1e-7


def on_both_faces(point):
    """Whether a point lies on the face x = 10 and on the face y = 10."""
    return abs(point[0] - SIDE) < TOLERANCE and abs(point[1] - SIDE) < TOLERANCE


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    edges = [edge for edge in cube.edges()
             if all(on_both_faces(vertex.point()) for vertex in edge.vertices())]
    assert len(edges) == 1, f"{len(edges)} edges at x = y = {SIDE}"
    # The radius runs linearly from the edge's start to its end: give each end its own.
    if edges[0].start_vertex().point()[2] < edges[0].end_vertex().point()[2]:
        start, end = RADIUS_BOTTOM, RADIUS_TOP
    else:
        start, end = RADIUS_TOP, RADIUS_BOTTOM
    return sm.variable_radius_fillet(cube, edges, start_radius=start, end_radius=end)
