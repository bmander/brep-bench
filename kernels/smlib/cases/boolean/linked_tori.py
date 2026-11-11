"""Two disjoint but linked tori, both of major radius 3 and tube radius 0.5: one at the origin
about z, one at (3, 0, 0) about y. Their union keeps two separate solids."""
import usd_brep as sm

MAJOR_RADIUS = 3
MINOR_RADIUS = 0.5
SECOND_CENTRE = (3, 0, 0)


def build():
    first = sm.create_torus((0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS)
    # A torus is made about z; a quarter turn about the line through its centre along x lays
    # its middle circle in the plane y = 0, about a line parallel to y.
    second = sm.create_torus(SECOND_CENTRE, MAJOR_RADIUS, MINOR_RADIUS)
    sm.rotate(second, SECOND_CENTRE, (1, 0, 0), 90)
    return sm.boolean_union(first, second)
