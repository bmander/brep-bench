"""A box: a rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and
the opposite corner at (10, 20, 30)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

CORNER = gp_Pnt(0, 0, 0)
OPPOSITE = gp_Pnt(10, 20, 30)


def build():
    return BRepPrimAPI_MakeBox(CORNER, OPPOSITE).Shape()
