"""Two crossed cylinders barely overlapping. Two solid cylinders of radius 5 and length 30 that
cross at right angles. The first lies along the x axis, from x = −15 to x = 15. The second runs
parallel to the y axis, from y = −15 to y = 15, with its axis on the line x = 0, z = 9.99. Make
the solid common to both: they overlap only between z = 4.99 and z = 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS = 5
LENGTH = 30
OFFSET = 9.99  # the second's axis is the line x = 0, z = 9.99


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-LENGTH / 2, 0, 0), gp.DX_s()),
                                     RADIUS, LENGTH).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -LENGTH / 2, OFFSET), gp.DY_s()),
                                      RADIUS, LENGTH).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the grazing cylinders")
    return common.Shape()
