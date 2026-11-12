"""A ball on a post, the concave edge between them chamfered with two distances. A post, the
cylinder of radius 3 on the z axis from z = 0 to z = 10, joined to a ball of radius 5 centered at
(0, 0, 10). The post's side meets the ball in the circle of radius 3 at z = 6, a concave edge.
Chamfer that edge with two distances: 2 down the post and 1 on the ball. A concave chamfer adds
material: it fills the groove between the post and the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the post on the circle at z = 4, and the ball on the
circle whose points are 1 in a straight line from the edge, the chord 1 long, turned 2·asin(1/10)
about the ball's center away from the post. That circle has radius ρ₁ = 147/50 + 6√11/25 ≈ 3.736 and
lies at z₁ = 152/25 + 9√11/50 ≈ 6.677. The chamfer is the cone between the two circles, and the
solid is the post and the ball joined, with the ring between them and the cone filled in.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight line
is what is meant.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import chamfered, is_ball_about

DOWN_POST, ON_BALL = 2, 1


def build():
    post = BRepPrimAPI_MakeCylinder(3, 10).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 5).Shape()
    joined = BRepAlgoAPI_Fuse(post, ball).Shape()
    # The concave edge through (0, −3, 6): 2 down the post, 1 on the ball.
    return chamfered(joined, (0, -3, 6), lambda f: not is_ball_about(f, (0, 0, 10)),
                     DOWN_POST, ON_BALL, "the edge where the post meets the ball")
