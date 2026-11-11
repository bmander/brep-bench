"""A plate drilled nine times. A plate from the origin to (100, 20, 10). Drill nine holes of
radius 2 through it along z, at y = 10 and x = 10, 20, 30, … 90. Drill them one after another,
each hole its own cut."""
import usd_brep as sm

LENGTH, WIDTH, THICKNESS = 100, 20, 10
HOLE_RADIUS = 2
HOLE_Y = 10
HOLE_XS = range(10, 100, 10)  # 10, 20, … 90


def build():
    plate = sm.create_box((0, 0, 0), LENGTH, WIDTH, THICKNESS)
    for x in HOLE_XS:
        # Each drill runs 1 past both faces of the plate.
        drill = sm.create_cylinder((x, HOLE_Y, -1), HOLE_RADIUS, THICKNESS + 2)
        plate = sm.boolean_difference(plate, drill)
    return plate
