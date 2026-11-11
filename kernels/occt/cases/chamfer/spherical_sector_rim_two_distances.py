"""A spherical sector whose rim is chamfered with two distances. A spherical sector: the part of a
ball of radius 5 centered on the origin that lies inside the cone with its apex at the origin,
about the z axis, opening upward with half angle α, cos α = 3/5 (about 53.13°). The cone's side
meets the ball in the circle of radius 4 at z = 3, the sector's rim. Chamfer the rim with two
distances: 2 down the cone and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the cone on the circle 2 down its side from the rim, of
radius 12/5 at z = 9/5, and the ball on the circle whose points are 1 in a straight line from the
rim, the chord 1 long, turned 2·asin(1/10) about the ball's center up from the rim. That circle has
radius ρ₁ = 98/25 − 9√11/50 ≈ 3.323 and lies at z₁ = 147/50 + 6√11/25 ≈ 3.736. The chamfer is the
cone between the two circles, and the solid is the sector cut by it.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight line
is what is meant.)"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere

from chamfer import chamfered, is_ball_about

DOWN_CONE, ON_BALL = 2, 1


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    # the cone with its apex at the origin, radius 8 at z = 6 (cos α = 3/5)
    cone = BRepPrimAPI_MakeCone(0, 8, 6).Shape()
    sector = BRepAlgoAPI_Common(ball, cone).Shape()
    # The rim through (0, −4, 3): 2 down the cone, 1 on the ball.
    return chamfered(sector, (0, -4, 3), lambda f: not is_ball_about(f, (0, 0, 0)),
                     DOWN_CONE, ON_BALL, "the sector's rim")
