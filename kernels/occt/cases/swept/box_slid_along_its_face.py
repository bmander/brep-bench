"""A box slid along the plane of its faces. The solid a box fills as it slides along a straight
line without turning. The box starts as the box from (0, 0, 0) to (10, 6, 4) and slides along the
vector (8, 5, 0), level, ending as the box from (8, 5, 0) to (18, 11, 4). Make the solid of
everything the box passes through.

The slide lies in the plane of the box's top and bottom, so those two faces slide within their own
planes: the result is a prism 4 high over the hexagon the rectangle from (0, 0) to (10, 6)
sweeps."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from sweep import slid, united

CORNER = (0, 0, 0)
FAR_CORNER = (10, 6, 4)
SLIDE = (8, 5, 0)


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(*CORNER), gp_Pnt(*FAR_CORNER)).Shape()
    # Every face is flat, so none needs splitting: the faces x = 10 and y = 6 lead the slide, the
    # faces x = 0 and y = 0 trail it, and the top and bottom slide within their own planes and
    # sweep nothing of their own (`slid` leaves them out: a prism of a face along its own plane
    # would have no volume).
    return united(slid(box, SLIDE))
