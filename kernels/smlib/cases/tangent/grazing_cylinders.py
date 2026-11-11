"""Grazing cylinders. Two cylinders of radius 5 and length 30 crossing at right angles, the first
along the x axis and the second parallel to y through x = 0, z = 9.99: what is common to both, a
cushion 0.01 thick."""
import usd_brep as sm

RADIUS = 5
HALF_LENGTH = 15
SECOND_Z = 9.99


def build():
    first = sm.create_cylinder((0, 0, -HALF_LENGTH), RADIUS, 2 * HALF_LENGTH)
    sm.rotate(first, (0, 0, 0), (0, 1, 0), 90)       # its axis from z to x
    second = sm.create_cylinder((0, 0, -HALF_LENGTH), RADIUS, 2 * HALF_LENGTH)
    sm.rotate(second, (0, 0, 0), (1, 0, 0), -90)     # its axis from z to y
    sm.translate(second, (0, 0, SECOND_Z))
    return sm.boolean_intersection(first, second)
