"""A torus tumbled through space. The solid a torus fills as it tumbles: it turns about one axis
while being watched from a frame that turns about another.

The torus has its centre at (3, 0, 2) and its axis parallel to the z axis; the circle through its
tube's centres has radius 1, and its tube has radius 0.5.

It moves by two turns at once, both driven by one parameter t:

- it turns by t/4 about the vertical line through (2, 0, 0), right-handed (from +x towards +y);
- the frame it is seen from turns by t about the line through (0, 0.5, 0.5) parallel to the
  x axis, right-handed (from +y towards +z).

Seen from that frame, the torus's place at t is its place turned by t/4 about the first line and
then by −t about the second. Make the solid of everything the torus passes through, so seen, as t
runs from −75° to 75°.

No turn keeps any line of the torus fixed, so the boundary is an envelope of no simpler kind."""
from bench import Refused


def build():
    # The torus moves by a turn about a vertical line followed by a turn about a line along x,
    # both driven by t: its axis of turning changes from instant to instant. OCCT's sweeps slide
    # (BRepPrimAPI_MakePrism), turn about one fixed axis (BRepPrimAPI_MakeRevol) or carry a shape
    # along a path under a law of its trihedron (BRepOffsetAPI_MakePipe, MakePipeShell), and none
    # of their laws turns a shape like this along its centre's path. And the solid's boundary is
    # the envelope of the torus under the motion, along characteristic curves that are none of
    # its circles, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid along a pair of turns about skew axes: it "
                  "slides, turns about one axis, or follows a path with a trihedron law, none "
                  "of which is this tumble; the boundary is the envelope of the tumbling torus, "
                  "which occt has no operation to find")
