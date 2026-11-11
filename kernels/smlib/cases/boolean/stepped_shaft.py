"""A stepped shaft on the z axis, made one solid. The first cylinder has radius 5 and runs from
z = 0 to z = 10. The second has radius 3 and runs from z = 10 to z = 20, standing on top of the
first."""
import usd_brep as sm

LOWER_RADIUS, UPPER_RADIUS = 5, 3
STEP_HEIGHT = 10  # each cylinder's length, and where the second stands


def build():
    lower = sm.create_cylinder((0, 0, 0), LOWER_RADIUS, STEP_HEIGHT)
    upper = sm.create_cylinder((0, 0, STEP_HEIGHT), UPPER_RADIUS, STEP_HEIGHT)
    return sm.boolean_union(lower, upper)
