"""Two half tori of one torus, overlapping. The halves z ≥ 0 and x ≥ 0 of the torus about the z
axis of radii 10 (its middle circle) and 3 (its tube), united; the second torus is turned a
quarter turn about z from the first."""
import usd_brep as sm

MAJOR_RADIUS = 10
MINOR_RADIUS = 3
REACH = MAJOR_RADIUS + MINOR_RADIUS + 1   # how far the blocks that halve the torus reach, past it


def build():
    # SMLib has no half-space solid, and its plane cut leaves an open shell: each half is what
    # the torus has in common with a block standing on the plane.
    above = sm.create_box((-REACH, -REACH, 0), 2 * REACH, 2 * REACH, REACH)
    upper = sm.boolean_intersection(sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS), above)

    torus = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    # SMLib's torus has a seam where its angle about z starts, on the +x side
    sm.rotate(torus, (0, 0, 0), (0, 0, 1), 90)
    ahead = sm.create_box((0, -REACH, -REACH), REACH, 2 * REACH, 2 * REACH)
    front = sm.boolean_intersection(torus, ahead)
    return sm.boolean_union(upper, front)
