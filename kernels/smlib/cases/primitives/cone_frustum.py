"""A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and
radius 2 at its top (z = 6)."""
import usd_brep as sm

BASE_CENTRE = (0, 0, 0)
BASE_RADIUS = 5
TOP_RADIUS = 2
HEIGHT = 6


def build():
    # A cone with two non-zero radii is SMLib's frustum, standing on its base's centre along +z.
    return sm.create_cone(BASE_CENTRE, BASE_RADIUS, TOP_RADIUS, HEIGHT)
