"""A ball pierced off centre. A ball of radius 20 centred on the origin, with a round hole of
radius 6 bored right through it parallel to the y axis. The hole's axis is the line x = 5, z = 0,
so the hole runs from x = −1 to x = 11 and stays inside the ball's outline: it enters and leaves
through the sphere and never breaks out of its side. If you cut the hole with a cylinder, make it
run past the ball at both ends."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

BALL_RADIUS = 20
HOLE_RADIUS = 6
HOLE_X = 5  # the hole's axis is x = 5, z = 0


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), BALL_RADIUS).Shape()
    # The drill runs along +y from y = −40 to y = 40, well past the ball at both ends.
    axis = gp_Ax2(gp_Pnt(HOLE_X, -2 * BALL_RADIUS, 0), gp.DY_s())
    drill = BRepPrimAPI_MakeCylinder(axis, HOLE_RADIUS, 4 * BALL_RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(ball, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
