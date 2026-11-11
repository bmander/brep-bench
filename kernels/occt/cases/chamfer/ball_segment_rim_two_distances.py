"""A ball cut flat above its middle, its rim chamfered with two distances. The part of a ball of
radius 5 centered on the origin that lies below the plane z = 3: a ball cut flat above its
middle. Its flat face is the disk of radius 4 at z = 3, and it meets the ball in that disk's rim
at an angle of about 126.9°, not square. Chamfer the rim with two distances: 2 on the flat face
and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the flat face on the circle of radius 2, and the ball
on the circle whose points are 1 in a straight line from the rim, the chord 1 long, turned
2·asin(1/10) about the ball's center down from the rim. That circle has radius ρ₁ = 98/25 + 9√11/50
≈ 4.517 and lies at z₁ = 147/50 − 6√11/25 ≈ 2.144. The chamfer is the cone between the two circles,
and the solid is the cut ball cut again by it.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight line
is what is meant.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import chamfered, is_ball_about

ON_FLAT, ON_BALL = 2, 1


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    below = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, -20), 40, 40, 23).Shape()
    cut_ball = BRepAlgoAPI_Common(ball, below).Shape()
    # The rim through (0, −4, 3): 2 on the flat face, 1 on the ball.
    return chamfered(cut_ball, (0, -4, 3), lambda f: not is_ball_about(f, (0, 0, 0)),
                     ON_FLAT, ON_BALL, "the cut ball's rim")
