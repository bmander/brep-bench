"""A hemisphere whose rim is chamfered with two distances. The upper half of a ball of radius 5
centered on the origin, z ≥ 0. Chamfer the circle where its flat face z = 0 meets the ball, with two
distances: 2 on the flat face and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the flat face on the circle of radius 3, 2 in from the
rim, and the ball on the circle whose points are 1 in a straight line from the rim, the chord 1
long. That chord turns 2·asin(1/10) about the ball's center, so the circle on the ball has radius
4.9 and height h = 3√11/10 ≈ 0.995. The chamfer is the cone between the two circles, and the solid
is the hemisphere cut by it: the flat face shrinks to the disk of radius 3, and the ball keeps
everything above z = h.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight line
is what is meant.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import chamfered, is_ball_about

ON_FLAT, ON_BALL = 2, 1


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), 40, 40, 20).Shape()
    hemisphere = BRepAlgoAPI_Common(ball, upper).Shape()
    # The rim through (0, −5, 0): 2 on the flat face, 1 on the ball.
    return chamfered(hemisphere, (0, -5, 0), lambda f: not is_ball_about(f, (0, 0, 0)),
                     ON_FLAT, ON_BALL, "the hemisphere's rim")
