"""A cube with every edge rounded. A cube of side 10 with all twelve of its edges rounded to
radius 1. Three rounds meet at each corner."""
import usd_brep as sm

SIDE = 10
RADIUS = 1


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    return sm.circular_fillet(cube, RADIUS)   # rounds every edge of the solid, in place
