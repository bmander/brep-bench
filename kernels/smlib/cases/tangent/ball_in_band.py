"""A ball in a band. A ball of radius 5 centred on the origin, united with a cylinder of the same
radius on the z axis from z = −3 to z = 3: the sphere touches the wall from inside along the
equator."""
import usd_brep as sm

RADIUS = 5
HALF_HEIGHT = 3


def build():
    ball = sm.create_sphere((0, 0, 0), RADIUS)
    band = sm.create_cylinder((0, 0, -HALF_HEIGHT), RADIUS, 2 * HALF_HEIGHT)
    return sm.boolean_union(ball, band)
