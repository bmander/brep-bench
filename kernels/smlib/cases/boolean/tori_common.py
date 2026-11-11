"""Two coaxial tori, their common part. Two solid tori about the z axis, each with a middle
circle of radius 5 and a tube of radius 2. The first middle circle lies in z = −1, the second in
z = 1. Make their common part."""
import usd_brep as sm

MAJOR_RADIUS = 5
MINOR_RADIUS = 2


def build():
    lower = sm.create_torus((0, 0, -1), MAJOR_RADIUS, MINOR_RADIUS)
    upper = sm.create_torus((0, 0, 1), MAJOR_RADIUS, MINOR_RADIUS)
    return sm.boolean_intersection(lower, upper)
