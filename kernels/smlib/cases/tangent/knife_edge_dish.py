"""A knife-edge dish. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, less a ball of
radius 5 about (0, 0, 10), the middle of its top face: the dish meets the wall tangentially along
the rim."""
import usd_brep as sm

RADIUS, HEIGHT = 5, 10


def build():
    cylinder = sm.create_cylinder((0, 0, 0), RADIUS, HEIGHT)
    ball = sm.create_sphere((0, 0, HEIGHT), RADIUS)
    return sm.boolean_difference(cylinder, ball)
