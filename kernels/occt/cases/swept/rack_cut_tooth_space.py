"""A tooth space cut by a rack. A gear blank with one tooth space cut by a rack tooth rolled round
it, as a gear is generated: module 2, 20 teeth, 20° pressure angle.

The blank is a disc of radius 22 about the y axis, from y = 0 to y = 6. Its pitch circle, in the
plane y = 0, has radius 20.

The rack's tooth is a prism whose section, in the plane y = 0, is the trapezoid symmetric about
the x axis with corners (18, ±0.842856) and (24, ±3.026666) in (x, z): its tip is 2 inside the
pitch circle, it is π wide where it crosses x = 20, and its flanks lean 20° from the x axis. The
prism runs from y = −2 to y = 8, through the blank with room to spare.

The rack moves so that its pitch line, the line x = 20 in the plane y = 0, rolls without slipping
on the blank's pitch circle, the tooth carried with it; as placed, the line touches the circle at
(20, 0, 0), the middle of the tooth. Seen from the blank, the rack rolls from 60° on one side of
that place to 60° on the other. Cut from the blank everything the tooth passes through.

Equivalently, as a machine does it: the blank turns about the y axis while the rack slides along
the pitch line, 2π·20 for every turn of the blank, over a third of a turn.

The space's flanks are involutes of the base circle (radius 20·cos 20°), and its root is the
curve the tooth's tip corners trace."""
from bench import Refused


def build():
    # OCCT's sweeps slide (BRepPrimAPI_MakePrism), turn (BRepPrimAPI_MakeRevol) or carry a shape
    # along a path under a law of its trihedron (BRepOffsetAPI_MakePipe, MakePipeShell). Seen
    # from the blank, the rolling rack does none of these: its frame would have to follow an
    # involute of the pitch circle, with a cusp where the tooth's middle touches, and that path
    # exists only if the implementation builds the involute from its formula. And swept face by
    # face, each flat flank would first have to be split along the line where the roll runs
    # along it, which moves across the flank: the flanks and root of the space are the envelope
    # of the tooth, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid along a rolling motion: it slides, turns, or "
                  "follows a path it is given, and the rack's path seen from the blank is an "
                  "involute the implementation would have to build from its formula; the "
                  "space's involute flanks and trochoid root are the envelope of the tooth, "
                  "which occt has no operation to find")
