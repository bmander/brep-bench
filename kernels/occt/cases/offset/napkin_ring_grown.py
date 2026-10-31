"""A napkin ring grown by 1. A ball of radius 5 centred on the origin, with a hole of radius 3
bored right through it along the z axis, grown by 1."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from offset import offset

BALL_RADIUS = 5
HOLE_RADIUS = 3
GROWTH = 1


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), BALL_RADIUS).Shape()
    # The drill runs well past the ball at both ends: from z = −10 to z = 10.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -2 * BALL_RADIUS), gp.DZ_s()),
                                     HOLE_RADIUS, 4 * BALL_RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(ball, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return offset(cut.Shape(), GROWTH)
