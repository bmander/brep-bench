"""A tube. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, with a hole of radius 3
bored along the axis. The cutting cylinder runs past both ends: from z = −1 to z = 11."""
import usd_brep as sm

OUTER_RADIUS = 5
INNER_RADIUS = 3
LENGTH = 10


def build():
    body = sm.create_cylinder((0, 0, 0), OUTER_RADIUS, LENGTH)
    bore = sm.create_cylinder((0, 0, -1), INNER_RADIUS, LENGTH + 2)
    return sm.boolean_difference(body, bore)
