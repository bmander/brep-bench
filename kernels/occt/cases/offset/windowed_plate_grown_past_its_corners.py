"""A windowed plate grown by 3, past its window's corner radius. The plate from (−20, −15, 0) to
(20, 15, 4) with a window cut right through it: the rectangle from (−10, −7) to (10, 7), its four
corners rounded to radius 2. Grown by 3, the window narrows and its rounded corners close up."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from draft import rounded_box
from offset import less, offset

PLATE = ((-20, -15, 0), (20, 15, 4))
WINDOW = ((-10, -7), (10, 7))
CORNER_RADIUS = 2
GROWTH = 3


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape()
    (x0, y0), (x1, y1) = WINDOW
    # The window's cutter runs 1 past both faces of the plate.
    cutter = rounded_box((x0, y0, PLATE[0][2] - 1), (x1, y1, PLATE[1][2] + 1), CORNER_RADIUS)
    return offset(less(plate, cutter, "the window"), GROWTH)
