"""A thin lens. Two balls of radius 5, about the origin and about (9.999999, 0, 0), overlapping to
a depth of 10⁻⁶: the lens common to both."""
import usd_brep as sm

RADIUS = 5
APART = 9.999999


def build():
    first = sm.create_sphere((0, 0, 0), RADIUS)
    second = sm.create_sphere((APART, 0, 0), RADIUS)
    return sm.boolean_intersection(first, second)
