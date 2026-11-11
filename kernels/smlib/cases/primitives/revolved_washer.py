"""A washer. In the xz plane, take the rectangle with x from 6 to 10 and z from 0 to 4. Revolve
it a full turn about the z axis."""
import usd_brep as sm

INNER, OUTER = 6, 10  # x
BOTTOM, TOP = 0, 4    # z


def build():
    corners = [(INNER, 0, BOTTOM), (OUTER, 0, BOTTOM), (OUTER, 0, TOP), (INNER, 0, TOP)]
    rectangle = [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]
    # About the line through the origin along +z; a turn of 360° closes the solid on itself.
    return sm.rotational_sweep(rectangle, (0, 0, 0), (0, 0, 1), 360)
