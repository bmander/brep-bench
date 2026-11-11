"""A pole cap a ten-thousandth high. A ball of radius 5 centred on the origin, in common with the
half-space above the plane z = 4.9999: a disk 0.063 across under a dome 10⁻⁴ high, around the
ball's pole."""
import usd_brep as sm

RADIUS = 5
CAP_HEIGHT = 1e-4
REACH = RADIUS + 1   # how far the block reaches, past the ball on every side but its floor


def build():
    # SMLib's sphere has its poles on the z axis
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    # the half-space, as a block whose floor is the plane
    floor = RADIUS - CAP_HEIGHT
    above = sm.create_box((-REACH, -REACH, floor), 2 * REACH, 2 * REACH, REACH - floor)
    return sm.boolean_intersection(ball, above)
