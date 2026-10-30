"""Two crossed cylinders, their common part. Two cylinders of radius 5 and length 30 that cross
at right angles. The first lies along the x axis, from x = −15 to 15; the second along the y
axis, from y = −15 to 15. Make the solid common to both. Their surfaces meet in two ellipses,
which cross each other at two points."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS = 5
LENGTH = 30


def build():
    along_x = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-15, 0, 0), gp.DX_s()),
                                       RADIUS, LENGTH).Shape()
    along_y = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -15, 0), gp.DY_s()),
                                       RADIUS, LENGTH).Shape()
    common = BRepAlgoAPI_Common(along_x, along_y)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the crossed cylinders")
    return common.Shape()
