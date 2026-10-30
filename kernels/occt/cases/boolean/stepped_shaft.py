"""A stepped shaft on the z axis, made one solid. The first cylinder has radius 5 and runs from
z = 0 to z = 10. The second has radius 3 and runs from z = 10 to z = 20, standing on top of the
first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


LOWER_RADIUS, UPPER_RADIUS = 5, 3
STEP_HEIGHT = 10  # each cylinder's length, and where the second stands


def build():
    lower = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()),
                                     LOWER_RADIUS, STEP_HEIGHT).Shape()
    upper = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, STEP_HEIGHT), gp.DZ_s()),
                                     UPPER_RADIUS, STEP_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(lower, upper)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cylinders")
    return fuse.Shape()
