"""Two coaxial cylinders overlapping. Two cylinders of radius 5 on the z axis, from z = 0 to
z = 10 and from z = 6 to z = 16, united: their walls coincide between z = 6 and z = 10."""
import usd_brep as sm

RADIUS, HEIGHT = 5, 10
SECOND_FROM = 6


def build():
    first = sm.create_cylinder((0, 0, 0), RADIUS, HEIGHT)
    second = sm.create_cylinder((0, 0, SECOND_FROM), RADIUS, HEIGHT)
    # SMLib's cylinder has a seam, on the +x side: the second's is turned a quarter turn away
    sm.rotate(second, (0, 0, 0), (0, 0, 1), 90)
    return sm.boolean_union(first, second)
