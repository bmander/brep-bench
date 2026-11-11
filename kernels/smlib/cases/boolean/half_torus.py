"""Half a torus. A torus about the z axis, centred on the origin, whose tube's centre circle has
radius 10 and whose tube has radius 2. Cut away everything where x < 0."""
import usd_brep as sm

MAJOR_RADIUS = 10
MINOR_RADIUS = 2


def build():
    torus = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    # SMLib's cut by a plane (sm.cut) trims the faces and leaves the body open, not a solid. So
    # "everything where x < 0" is cut away as a block: one with a face in the plane x = 0 that
    # holds all of the torus on that side, with a unit to spare.
    reach = MAJOR_RADIUS + MINOR_RADIUS + 1
    negative_x = sm.create_box((-reach, -reach, -MINOR_RADIUS - 1),
                               reach, 2 * reach, 2 * MINOR_RADIUS + 2)
    return sm.boolean_difference(torus, negative_x)
