"""An L-shaped plate. Its outline in the xy plane is the polygon through (0, 0), (20, 0),
(20, 5), (5, 5), (5, 15) and (0, 15). Extrude it 8 along +z."""
import usd_brep as sm

OUTLINE = [(0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15)]
THICKNESS = 8


def build():
    corners = [(x, y, 0) for x, y in OUTLINE]
    # The closed outline, one line segment per side, head to tail.
    sides = [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]
    # A closed planar loop swept along a line, capped at both ends, is a solid.
    return sm.linear_sweep(sides, (0, 0, 1), THICKNESS)
