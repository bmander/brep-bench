"""A tube. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, with a hole of radius 3
bored along the axis. The cutting cylinder runs past both ends: from z = −1 to z = 11."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

OUTER_RADIUS = 5
INNER_RADIUS = 3
LENGTH = 10


def build():
    body = BRepPrimAPI_MakeCylinder(OUTER_RADIUS, LENGTH).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()),
                                    INNER_RADIUS, LENGTH + 2).Shape()
    cut = BRepAlgoAPI_Cut(body, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    return cut.Shape()
