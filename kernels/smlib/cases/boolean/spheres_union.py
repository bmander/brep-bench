"""Two balls united. Two balls of radius 5, one centred on the origin and one at (6, 0, 0). Make
their union."""
import usd_brep as sm

RADIUS = 5


def build():
    first = sm.create_sphere((0, 0, 0), RADIUS)
    second = sm.create_sphere((6, 0, 0), RADIUS)
    return sm.boolean_union(first, second)
