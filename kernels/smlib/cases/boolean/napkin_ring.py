"""A napkin ring. A ball of radius 5 centred on the origin, with a hole of radius 3 bored right
through it along the z axis."""
import usd_brep as sm

BALL_RADIUS = 5
HOLE_RADIUS = 3


def build():
    ball = sm.create_sphere((0, 0, 0), BALL_RADIUS)
    # The drill runs well past the ball at both ends: from z = −10 to z = 10.
    drill = sm.create_cylinder((0, 0, -2 * BALL_RADIUS), HOLE_RADIUS, 4 * BALL_RADIUS)
    return sm.boolean_difference(ball, drill)
