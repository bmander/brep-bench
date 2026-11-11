"""A torus in a drum. A torus about the z axis of radii 10 (its middle circle) and 3 (its tube),
united with a cylinder of radius 13 on the z axis from z = −1 to z = 1: the torus touches the
drum's wall from inside along its outer equator."""
import usd_brep as sm

MAJOR_RADIUS = 10
MINOR_RADIUS = 3
DRUM_RADIUS = MAJOR_RADIUS + MINOR_RADIUS
DRUM_HALF_HEIGHT = 1


def build():
    torus = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    drum = sm.create_cylinder((0, 0, -DRUM_HALF_HEIGHT), DRUM_RADIUS, 2 * DRUM_HALF_HEIGHT)
    return sm.boolean_union(torus, drum)
