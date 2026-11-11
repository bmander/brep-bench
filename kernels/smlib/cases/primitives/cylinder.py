"""A cylinder: radius 5 and height 10, its axis the z axis, standing on the xy plane (it runs
from z = 0 to z = 10)."""
import usd_brep as sm

BASE_CENTRE = (0, 0, 0)
RADIUS = 5
HEIGHT = 10


def build():
    # The cylinder stands on its base's centre along +z.
    return sm.create_cylinder(BASE_CENTRE, RADIUS, HEIGHT)
