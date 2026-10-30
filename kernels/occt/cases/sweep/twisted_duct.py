"""A square duct turned to a diamond. A solid duct running along the y axis from y = 0 to y = 40,
with a square at its start and a diamond at its end, each side a twisted ruled face.

At y = 0 the section is the square with corners A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10)
and D (10, 0, 10). At y = 40 it is the diamond with corners A′ (0, 40, −12), B′ (−12, 40, 0),
C′ (0, 40, 12) and D′ (12, 40, 0): its corners are the square's turned an eighth of a turn about
the y axis and pushed out from 10√2 to 12. Each corner of the square runs in a straight line to
the corner of the diamond with the same letter (A to A′, B to B′, C to C′, D to D′), and at every
y between the section is the quadrilateral through the four points those lines reach there.

So each of the four long faces is the ruled surface swept by the straight segment joining two
neighbouring corner lines at the same y: the face through A, B, B′ and A′ is made of the
segments from A + t(A′ − A) to B + t(B′ − B) for t from 0 to 1, and so on round. Since the
square's sides are not parallel to the diamond's, these faces are not flat. The two ends are
flat."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused

SQUARE = [(10, 0, -10), (-10, 0, -10), (-10, 0, 10), (10, 0, 10)]  # A, B, C, D
DIAMOND = [(0, 40, -12), (-12, 40, 0), (0, 40, 12), (12, 40, 0)]  # A′, B′, C′, D′


def polygon(corners):
    poly = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        poly.Add(gp_Pnt(*corner))
    poly.Close()
    return poly.Wire()


def build():
    # A ruled loft joins the two wires edge to edge: each wire starts at its A and runs the same
    # way round, and the check that would re-pair the vertices is off, so A meets A′, B meets B′,
    # and each side is the ruled surface between corresponding edges.
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.CheckCompatibility(False)
    loft.AddWire(polygon(SQUARE))
    loft.AddWire(polygon(DIAMOND))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the square to the diamond")
    return loft.Shape()
