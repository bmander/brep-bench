"""A lens of two balls whose rim is chamfered with two distances. The lens common to two balls of
radius 5, one centered at (0, 0, 3) and one at (0, 0, −3). Its upper face is part of the ball about
(0, 0, −3) and its lower face part of the ball about (0, 0, 3); they meet in its rim, the circle of
radius 4 at z = 0. Chamfer the rim with two distances: 2 on the upper face and 1 on the lower.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets each face on the circle whose points are that far in a
straight line from the rim, the chord turned 2·asin(d/10) about that face's ball's center. On the
upper face that circle has radius ρ₂ = 92/25 − 12√6/25 ≈ 2.504 and lies at z₂ = 16√6/25 − 6/25 ≈
1.328; on the lower face, radius ρ₁ = 98/25 − 9√11/50 ≈ 3.323 at z₁ = 3/50 − 6√11/25 ≈ −0.736. The
chamfer is the cone between the two circles, and the solid is the lens cut by it.

(Measured along a ball as an arc, d would stop about d³/600 short of its circle: the straight line
is what is meant.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import chamfered, is_ball_about

ON_UPPER, ON_LOWER = 2, 1


def build():
    above = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 3), 5).Shape()
    below = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, -3), 5).Shape()
    lens = BRepAlgoAPI_Common(above, below).Shape()
    # The rim through (0, −4, 0): 2 on the upper face, which lies on the ball about (0, 0, −3).
    return chamfered(lens, (0, -4, 0), lambda f: is_ball_about(f, (0, 0, -3)),
                     ON_UPPER, ON_LOWER, "the lens's rim")
