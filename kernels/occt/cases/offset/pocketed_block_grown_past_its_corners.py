"""A pocketed block grown by 3, past its pocket's corner radius. The block from (−20, −15, 0) to
(20, 15, 10) with a pocket cut down into its top: the rectangle from (−10, −7) to (10, 7), its four
corners rounded to radius 2, from the top down to a flat floor at z = 4. Grown by 3, the pocket's
rounded corners close up, and over its rim the grown top dips."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from draft import rounded_box
from offset import less, offset

BLOCK = ((-20, -15, 0), (20, 15, 10))
POCKET = ((-10, -7), (10, 7))
CORNER_RADIUS = 2
FLOOR = 4
GROWTH = 3


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape()
    (x0, y0), (x1, y1) = POCKET
    # The pocket's cutter runs 1 above the block's top.
    cutter = rounded_box((x0, y0, FLOOR), (x1, y1, BLOCK[1][2] + 1), CORNER_RADIUS)
    return offset(less(block, cutter, "the pocket"), GROWTH)
