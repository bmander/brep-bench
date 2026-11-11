"""A disc with six holes indexed around it. A round disc of radius 20 on the z axis, from z = −5
to z = 0. Six round holes of radius 2 go straight through it along z, their axes on the circle of
radius 12 about the z axis: the first at (12, 0), and each of the others the first turned about
the z axis by 60°, 120°, 180°, 240° and 300°. Cut them one after another, each hole its own cut.
If you cut a hole with a cylinder, make it run past both faces of the disc."""
import usd_brep as sm

DISC_RADIUS = 20
BOTTOM, TOP = -5, 0
HOLE_RADIUS = 2
PITCH_RADIUS = 12
COUNT = 6


def build():
    disc = sm.create_cylinder((0, 0, BOTTOM), DISC_RADIUS, TOP - BOTTOM)
    # The first drill, at (12, 0), runs 1 past both faces: from z = −6 to z = 1.
    first = sm.create_cylinder((PITCH_RADIUS, 0, BOTTOM - 1), HOLE_RADIUS, TOP - BOTTOM + 2)
    for i in range(COUNT):
        # A boolean consumes its operands and rotate turns its target in place, so each drill
        # is a copy of the first, turned about the z axis.
        drill = sm.rotate(first.copy(), (0, 0, 0), (0, 0, 1), i * 360 / COUNT)
        disc = sm.boolean_difference(disc, drill)
    return disc
