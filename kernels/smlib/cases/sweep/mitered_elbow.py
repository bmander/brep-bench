"""A square bar mitred around a sharp corner. The path: two straight runs, from (0, 0, 0) along +x
to (40, 0, 0), then turning 120° to the left (toward +y) and on 40 to (20, 20√3, 0). The section:
a square 10 × 10, square to the path and centred on it, two of its sides horizontal. Each run is
the square swept straight along it, and at the corner the two are mitred: they meet in the
vertical plane through the corner that bisects it. The bar's ends are flat and square to the
path."""
import math

import usd_brep as sm

RUN, HALF_SIDE = 40, 5
TURN = math.radians(120)   # to the left, from +x toward +y


def polyline(points, closed=False):
    """Straight lines from each point to the next, and back to the first if `closed`."""
    points = points + points[:1] if closed else points
    return [sm.create_line_segment(a, b) for a, b in zip(points, points[1:])]


def build():
    path = polyline([(0, 0, 0), (RUN, 0, 0), (RUN + RUN * math.cos(TURN), RUN * math.sin(TURN), 0)])
    # The square at the path's start, in the plane x = 0, its sides at y, z = ±5.
    h = HALF_SIDE
    square = polyline([(0, -h, -h), (0, h, -h), (0, h, h), (0, -h, h)], closed=True)
    # A path in one plane is swept with its corners mitred; the section rides the path by the
    # middle of its bounding box, here the square's own middle.
    return sm.sweep_along_planar_path(square, path)
