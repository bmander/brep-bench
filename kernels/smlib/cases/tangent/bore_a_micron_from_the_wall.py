"""A bore a micron from the wall. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, with
a hole of radius 4 bored through it along the line x = 0.999999, y = 0: the wall between them is
10⁻⁶ thick at (5, 0, z)."""
import usd_brep as sm

RADIUS, HEIGHT = 5, 10
HOLE_RADIUS = 4
HOLE_X = 0.999999
PAST = 1   # how far the cutter runs past each end


def build():
    cylinder = sm.create_cylinder((0, 0, 0), RADIUS, HEIGHT)
    cutter = sm.create_cylinder((HOLE_X, 0, -PAST), HOLE_RADIUS, HEIGHT + 2 * PAST)
    return sm.boolean_difference(cylinder, cutter)
