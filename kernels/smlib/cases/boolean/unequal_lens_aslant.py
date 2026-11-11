"""An unequal lens, aslant. Two balls: one of radius 5.511 centred on the origin, and one of
radius 3.317 centred at (4.231, 3.467, 2.653), so that they overlap but neither holds the other.
Make the lens common to both."""
import usd_brep as sm


def build():
    first = sm.create_sphere((0, 0, 0), 5.511)
    second = sm.create_sphere((4.231, 3.467, 2.653), 3.317)
    return sm.boolean_intersection(first, second)
