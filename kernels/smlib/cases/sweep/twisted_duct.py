"""A square duct turned to a diamond. A solid duct running along the y axis from y = 0 to y = 40.
At y = 0 its section is the square with corners A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10)
and D (10, 0, 10); at y = 40 the diamond with corners A′ (0, 40, −12), B′ (−12, 40, 0),
C′ (0, 40, 12) and D′ (12, 40, 0). Each corner of the square runs in a straight line to the corner
of the diamond with the same letter, and each of the four long faces is the ruled surface between
two neighbouring corner lines. The two ends are flat."""
import usd_brep as sm

SQUARE = [(10, 0, -10), (-10, 0, -10), (-10, 0, 10), (10, 0, 10)]
DIAMOND = [(0, 40, -12), (-12, 40, 0), (0, 40, 12), (12, 40, 0)]


def outline(corners):
    """The closed polygon through `corners`, a side from each to the next."""
    return [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]


def build():
    # A skin of degree 1 joins each side of the square to the diamond's side in the same place by
    # straight lines, A to A′ and so on, and caps both ends.
    return sm.create_skin_primitive([4, 4], outline(SQUARE) + outline(DIAMOND), degree=1)
