"""A wheel cut by a plus always in mesh with it. A wheel cut by a plus-shaped tool that turns in
mesh with it, as a gear shaper cuts.

The tool is a plus in the plane z = 0, centred on the origin: four arms along +x, +y, −x and −y,
each 30 long from the centre and 8 wide, joined in a square 8 by 8 at the centre. It is extruded
from z = −10 to z = 0.

The wheel's blank is a disc of radius 40 about the vertical line through (−50, 0), from z = −8
to z = −2.

The tool turns about the z axis while the wheel turns two thirds as fast the other way about its
own axis: when the tool has turned right-handed by t (from +x towards +y), the wheel has turned by
−2t/3 about the vertical line through (−50, 0). Over t from 0° to 540° the wheel makes one whole
turn and the tool one and a half, after which each stands as it began. Cut from the wheel
everything the tool passes through, as seen from the wheel: what is left is the material that at
every instant of the motion lies outside the tool.

The tool is never clear of the wheel."""
from bench import Refused


def build():
    # Seen from the wheel, the tool turns about its own axis while that axis is carried round
    # the wheel's: two turns at once, in the ratio 5 : 2. OCCT's sweeps slide
    # (BRepPrimAPI_MakePrism), turn about one axis (BRepPrimAPI_MakeRevol) or carry a shape along
    # a path under a law of its trihedron (BRepOffsetAPI_MakePipe, MakePipeShell); the pair of
    # turns is none of them, since along the circle the tool's centre runs on, the tool turns
    # 5/2 as fast as the circle's own frame. And swept face by face, each flat side of the plus
    # would have to be split where the motion runs along it, a line that moves across it: the
    # wheel's walls are the envelope of the plus, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid along a pair of turns: it slides, turns about "
                  "one axis, or follows a path with its tangent, and the plus seen from the "
                  "wheel turns 5/2 as fast as the circle its centre runs on; the wheel's walls "
                  "are the envelope of the plus, which occt has no operation to find")
