"""A block with a hemispherical pocket. A block from (−10, −10, −10) to (10, 10, 0). Cut from it
a ball of radius 5 centred on the origin, leaving a hemispherical pocket in its top face."""
import usd_brep as sm

BALL_RADIUS = 5


def build():
    block = sm.create_box((-10, -10, -10), 20, 20, 10)
    ball = sm.create_sphere((0, 0, 0), BALL_RADIUS)
    return sm.boolean_difference(block, ball)
