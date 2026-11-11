"""An octahedron with every edge rounded, four rounds meeting at each corner. The octahedron
|x| + |y| + |z| ≤ 10, its six corners on the axes, 10 from the origin. Round all twelve of its
edges to radius 2: at each corner the ball that touches all four faces there closes the corner
with a patch of its sphere."""
import math

import usd_brep as sm

REACH = 10   # each corner's distance from the origin
RADIUS = 2


def build():
    # Two square pyramids base to base. A pyramid's base is square to the x and y axes: its
    # corners are turned onto them.
    def pyramid():
        return sm.rotate(sm.create_pyramid((0, 0, 0), REACH * math.sqrt(2), REACH),
                         (0, 0, 0), (0, 0, 1), 45)
    upper = pyramid()
    lower = sm.rotate(pyramid(), (0, 0, 0), (1, 0, 0), 180)
    octahedron = sm.boolean_union(upper, lower)
    assert octahedron.edge_count() == 12, f"{octahedron.edge_count()} edges"
    return sm.circular_fillet(octahedron, RADIUS)   # rounds every edge, in place
