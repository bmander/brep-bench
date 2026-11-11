"""A quarter of a washer. In the xz plane, take the rectangle with x from 6 to 10 and z from 0
to 4. Revolve it 90° about the z axis, right-handed: from +x towards +y."""
import usd_brep as sm

INNER, OUTER = 6, 10  # x
BOTTOM, TOP = 0, 4    # z
ANGLE = 90            # degrees


def build():
    corners = [(INNER, 0, BOTTOM), (OUTER, 0, BOTTOM), (OUTER, 0, TOP), (INNER, 0, TOP)]
    rectangle = [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]
    # About the line through the origin along +z; the ends of a partial turn are capped.
    return sm.rotational_sweep(rectangle, (0, 0, 0), (0, 0, 1), ANGLE)
