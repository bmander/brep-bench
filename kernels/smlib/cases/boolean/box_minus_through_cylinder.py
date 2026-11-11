"""A cube drilled through. Take a cube of side 10 between the origin and (10, 10, 10). Drill a
hole of radius 3 straight through it along z, centred at x = 5, y = 5. The cutting cylinder runs
past both faces: from z = −1 to z = 11."""
import usd_brep as sm

SIDE = 10
HOLE_RADIUS = 3
HOLE_FROM, HOLE_TO = -1, 11  # z, past both faces


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    # A cylinder stands on its base's centre along +z.
    drill = sm.create_cylinder((5, 5, HOLE_FROM), HOLE_RADIUS, HOLE_TO - HOLE_FROM)
    return sm.boolean_difference(cube, drill)
