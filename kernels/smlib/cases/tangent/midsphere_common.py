"""A cube and its midsphere, in common. A cube of side 10 centred on the origin, and a ball about
the origin of radius 5√2, whose sphere touches each of the cube's twelve edges at its midpoint:
what is common to both."""
import usd_brep as sm

HALF = 5
RADIUS = HALF * 2 ** 0.5   # from the centre to the middle of an edge


def build():
    cube = sm.create_box((-HALF, -HALF, -HALF), 2 * HALF, 2 * HALF, 2 * HALF)
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    return sm.boolean_intersection(cube, ball)
