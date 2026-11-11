"""An octahedron with every edge chamfered, four chamfers meeting at each corner. The octahedron
|x| + |y| + |z| ≤ 10, its six corners on the axes, 10 from the origin. Chamfer all twelve of its
edges, each with the same distance √6 on both of its faces, measured across the face square to
the edge: the chamfer of the edge from (10, 0, 0) to (0, 10, 0) is the plane x + y = 8. Four
chamfers meet at each corner, and the corner is cut flat by a square: 26 flat faces."""
import math

import usd_brep as sm

REACH = 10   # each corner's distance from the origin
DISTANCE = math.sqrt(6)
# The flat section joins the lines where a round would touch the two faces, and the kernel takes
# the round's radius: those lines lie at the radius over the tangent of half the angle between
# the faces from the edge. An octahedron's faces meet at arccos(−1/3).
RADIUS = DISTANCE * math.tan(math.acos(-1 / 3) / 2)
LINEAR = 0   # fillet_edges' flat cross-section: the round's two tangent lines joined by a plane


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
    # All twelve edges blended at once with the flat section, each corner closed by its own flat
    # face. (chamfer_fillet, the kernel's chamfer of every edge, mitres a cube's corners, and
    # declines the octahedron.)
    return sm.fillet_edges(solid, solid.edges(), radius=RADIUS, xsect_type=LINEAR)
