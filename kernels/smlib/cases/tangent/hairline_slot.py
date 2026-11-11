"""A hairline slot. A cube of side 10 between the origin and (10, 10, 10), with a slot a millionth
wide cut down into it from the top: between x = 5 and x = 5.000001, right through in y, from
z = 5 up."""
import usd_brep as sm

SIDE = 10
SLOT_X, SLOT_WIDTH = 5, 1e-6
SLOT_FLOOR = 5
PAST = 1   # how far the cutter runs past the cube's front, back and top


def build():
    cube = sm.create_box((0, 0, 0), SIDE, SIDE, SIDE)
    cutter = sm.create_box((SLOT_X, -PAST, SLOT_FLOOR),
                           SLOT_WIDTH, SIDE + 2 * PAST, SIDE - SLOT_FLOOR + PAST)
    return sm.boolean_difference(cube, cutter)
