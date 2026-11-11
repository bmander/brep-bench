"""A Reuleaux tetrahedron. Four balls of radius 10, centred at the vertices of a regular
tetrahedron of side 10: (0, 0, 0), (10, 0, 0), (5, 5√3, 0) and (5, 5√3/3, 10√(2/3)). Make the
solid common to all four balls."""
from math import sqrt

import usd_brep as sm

CENTERS = [(0, 0, 0), (10, 0, 0), (5, 5 * sqrt(3), 0),
           (5, 5 * sqrt(3) / 3, 10 * sqrt(2 / 3))]
RADIUS = 10


def build():
    solid = sm.create_sphere(CENTERS[0], RADIUS)
    for center in CENTERS[1:]:
        solid = sm.boolean_intersection(solid, sm.create_sphere(center, RADIUS))
    return solid
