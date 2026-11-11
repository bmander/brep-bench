"""A ball: a solid ball of radius 5 centred on the origin."""
import usd_brep as sm

CENTRE = (0, 0, 0)
RADIUS = 5


def build():
    return sm.create_sphere(CENTRE, RADIUS)
