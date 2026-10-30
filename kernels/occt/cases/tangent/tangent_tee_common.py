"""A tangent tee, its common part. Two solid cylinders crossing at right angles. The first, of
radius 5, lies along the x axis from x = −15 to x = 15. The second, of radius 3, runs parallel to
the y axis from y = −15 to y = 15, its axis raised to the line x = 0, z = 2. Make the solid common
to both. The tops of both are at z = 5, where the surfaces are tangent."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

FIRST_RADIUS = 5
SECOND_RADIUS = 3
SECOND_Z = 2  # the second's axis is the line x = 0, z = 2
LENGTH = 30


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-LENGTH / 2, 0, 0), gp.DX_s()),
                                     FIRST_RADIUS, LENGTH).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -LENGTH / 2, SECOND_Z), gp.DY_s()),
                                      SECOND_RADIUS, LENGTH).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the tangent tee")
    return common.Shape()
