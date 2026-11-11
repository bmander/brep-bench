"""A torus through a square slab. A torus about the z axis of radii 10 (its middle circle) and 3
(its tube), united with a slab from (−13, −13, −1) to (13, 13, 1): the torus's outer equator
touches each of the slab's four sides at its middle."""
import usd_brep as sm

MAJOR_RADIUS = 10
MINOR_RADIUS = 3
SLAB_HALF = MAJOR_RADIUS + MINOR_RADIUS
SLAB_HALF_HEIGHT = 1


def build():
    torus = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    # SMLib's torus has a seam where its angle about z starts, on the +x side, through the point
    # of contact (13, 0, 0): it is turned 45° away
    sm.rotate(torus, (0, 0, 0), (0, 0, 1), 45)
    slab = sm.create_box((-SLAB_HALF, -SLAB_HALF, -SLAB_HALF_HEIGHT),
                         2 * SLAB_HALF, 2 * SLAB_HALF, 2 * SLAB_HALF_HEIGHT)
    return sm.boolean_union(torus, slab)
