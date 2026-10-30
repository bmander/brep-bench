"""Two equal cylinders overlapping end to end. Two cylinders of radius 5 on the z axis, the first
from z = 0 to z = 10 and the second from z = 6 to z = 16. Unite them. If the cylinders have a
seam, turn the second a quarter turn about the z axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 10
SECOND_BOTTOM = 6


def build():
    first = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    # OCCT's cylinder has its seam along its frame's x direction: this one's runs along y
    turned = gp_Ax2(gp_Pnt(0, 0, SECOND_BOTTOM), gp.DZ_s(), gp.DY_s())
    second = BRepPrimAPI_MakeCylinder(turned, RADIUS, HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cylinders")
    return fuse.Shape()
