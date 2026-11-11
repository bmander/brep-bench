"""A hemisphere. A ball of radius 5 centred on the origin, with everything below the xy plane
cut away."""
import usd_brep as sm

RADIUS = 5


def build():
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    # SMLib's cut by a plane (sm.cut) trims the faces and leaves the body open, not a solid. So
    # everything below the xy plane is cut away as a block: one with its top in that plane that
    # holds the lower half of the ball, with a unit to spare.
    reach = RADIUS + 1
    below = sm.create_box((-reach, -reach, -reach), 2 * reach, 2 * reach, reach)
    return sm.boolean_difference(ball, below)
