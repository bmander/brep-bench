"""A hollow square reducer. A hollow square tube that tapers along the y axis, from y = 0 to
y = 40. Its sections are squares centred on the y axis, sides parallel to x and z, each with a
square hole at its centre leaving a wall 2 thick: at y = 0 the outside is 24 × 24 and the hole
20 × 20; at y = 40 the outside is 12 × 12 and the hole 8 × 8. Each corner, outside and in, runs in
a straight line from one end to the other. The two ends are flat."""
import usd_brep as sm

LENGTH = 40
HALF_START, HALF_END = 12, 6   # the outside's half-width at y = 0 and at y = 40
WALL = 2


def square(half, y):
    """The square of half-width `half` about the y axis in the plane at `y`, as four lines."""
    corners = [(half, y, -half), (-half, y, -half), (-half, y, half), (half, y, half)]
    return [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]


def section(half, y):
    """The hollow section at `y`: its outside, then its hole."""
    return square(half, y) + square(half - WALL, y)


def build():
    # A skin of degree 1 between the two sections, each an outer loop and an inner one: every
    # side runs straight to the side in the same place at the other end, and the ends are capped.
    return sm.create_skin_primitive([8, 8], section(HALF_START, 0) + section(HALF_END, LENGTH),
                                    degree=1)
