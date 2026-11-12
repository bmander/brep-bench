"""A bored ball whose rim is chamfered with two distances. A ball of radius 5 centered on the
origin, with a hole of radius 2 bored right through it along the z axis. The bore meets the ball in
two circles of radius 2, at z = ±√21. Chamfer the upper one, at z = √21 ≈ 4.583, with two distances:
2 down the bore and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the bore on the circle at z = √21 − 2, and the ball on
the circle whose points are 1 in a straight line from the rim, the chord 1 long, turned 2·asin(1/10)
about the ball's center away from the bore. That circle has radius ρ₁ = 49/25 + 3√231/50 ≈ 2.872 and
lies at z₁ = 49√21/50 − 3√11/25 ≈ 4.093. The chamfer is the cone between the two circles, and the
solid is the bored ball cut by it. The lower rim stays sharp.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight line
is what is meant.)"""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from chamfer import chamfered, is_ball_about

DOWN_BORE, ON_BALL = 2, 1


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    # The bore runs past both sides of the ball.
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp_Dir(0, 0, 1)), 2, 20).Shape()
    bored = BRepAlgoAPI_Cut(ball, bore).Shape()
    # The upper rim through (0, −2, √21): 2 down the bore, 1 on the ball.
    return chamfered(bored, (0, -2, math.sqrt(21)), lambda f: not is_ball_about(f, (0, 0, 0)),
                     DOWN_BORE, ON_BALL, "the bored ball's upper rim")
