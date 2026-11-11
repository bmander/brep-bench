"""A wedge a millionth steep. A cube of side 10 from (−5, −5, −10) to (5, 5, 0), in common with the
half-space above the plane z = y·10⁻⁶, which crosses the top face along its middle line, the x
axis: a wedge 5·10⁻⁶ thick at y = −5, thinning to an edge at y = 0."""
from math import atan, degrees

import usd_brep as sm

HALF = 5
SLOPE = 1e-6
REACH = 2 * HALF   # how far the block standing on the plane reaches, past the cube


def build():
    cube = sm.create_box((-HALF, -HALF, -2 * HALF), 2 * HALF, 2 * HALF, 2 * HALF)
    # SMLib has no half-space solid, and its plane cut leaves an open shell: the half-space is a
    # block standing on the plane z = 0, tipped about the x axis to the plane's slope
    above = sm.create_box((-REACH, -REACH, 0), 2 * REACH, 2 * REACH, REACH)
    sm.rotate(above, (0, 0, 0), (1, 0, 0), degrees(atan(SLOPE)))
    return sm.boolean_intersection(cube, above)
