"""A straight bevel gear's tooth space cut by its pinion. A bevel gear blank with one tooth space cut
by a tooth of its pinion turning in mesh with it, as a gear shaper cuts, but on shafts that meet: a
30-tooth gear and a 15-tooth pinion, module 2 at the outer end, the shafts square.

This is the first of five cases that cut a gear with its mate in one way while the shafts move
apart, from bevel to worm: this one, the hypoid, the spiroid, the helicon and the worm wheel. In
each the blank turns about the z axis while the tool turns about a line square to it, at the pair's
ratio, and what the tool passes through is cut from the blank. Here the two axes meet.

The gear's axis is the z axis and the pinion's the x axis; they meet at the origin O. Write g = (2,
0, 1)/√5 and d = (−1, 0, 2)/√5. The line through O along g is where the two pitch cones touch: the
gear's pitch cone is the cone about the z axis through it (half-angle atan 2 ≈ 63.43°), and the
pinion's the cone about the x axis through it (half-angle atan ½). d is square to g in the xz-plane
and points from the pinion into the gear.

The blank is the solid of revolution, about the z axis, of the quadrilateral in the xz-plane (ρ =
x, z) whose corners are the points s·g + u·d at

- s = 15√5, u = −2: (30.894427, 13.211146), the outer tip;
- s = 15√5, u = 6: (27.316718, 20.366563);
- s = 15√5 − 10, u = 6: (18.372447, 15.894427);
- s = 15√5 − 10, u = −2(15√5 − 10)/(15√5): (21.683489, 9.272343), the inner tip.

So its back and front faces are cones square to the pitch cone, at cone distances 15√5 ≈ 33.54
(where the pitch radius is 30) and 10 less; its face cone runs from the outer tip to O, 2 outside
the pitch cone at the outer end; and its bore is the cone 6 inside the pitch cone.

The tool is one tooth of the pinion, straight-sided, its flanks planes through O. In the plane
square to g through the outer pitch point P = 15√5·g = (30, 0, 15), take the trapezoid of the
points P + u·d + w·(0, 1, 0) with |u| ≤ 2.5 and |w| ≤ π/2 − u·tan 20°. Its corners are (28.881966,
±0.660871, 17.236068), 2.5 into the gear, and (31.118034, ±2.480721, 12.763932), 2.5 into the
pinion: a tooth π wide at the pitch cone, its sides 20° from d. The tooth is the pyramid with apex
O over this trapezoid, cut off by the planes square to g at distances 15√5 − 12 and 15√5 + 2 from
O: a frustum of a pyramid, reaching 2 past the blank at each end. It is centred on the line where
the pitch cones touch.

The motion. The pinion turns by t about the x axis, right-handed (from +y toward +z), while the
gear turns by −t/2 about the z axis, so that the pitch cones roll on each other without slipping.
Seen from the blank, the tooth at t is turned by t about the x axis and then by t/2 about the z
axis. As t runs from −60° to 60° the tooth comes into the blank, passes through the mesh and leaves
it; at both ends it is clear of the blank. Cut from the blank everything the tooth passes through,
seen from the blank.

Both turns hold O still and the tooth's flanks pass through O, so between the tooth's ends the
space is a cone with apex O: its section by every sphere about O is the same curve, scaled. That
curve is the envelope of an arc of a great circle as the two turns carry it over the sphere, a
spherical curve and not a plane involute. The gear it makes is conjugate to this straight-sided
tooth, not to an involute pinion."""
from bench import Refused


def build():
    # OCCT's sweeps slide (BRepPrimAPI_MakePrism), turn (BRepPrimAPI_MakeRevol) or carry a shape
    # along a path under a law of its trihedron (BRepOffsetAPI_MakePipe, MakePipeShell). Seen from
    # the gear, the pinion's tooth turns about the x axis while that axis turns about the z axis:
    # a spherical motion, which none of these makes. The space's flanks are the envelope of the
    # tooth's flank planes under it, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid under two turns at once: it slides, turns about "
                  "one axis, or follows a path it is given, and the tooth seen from the gear "
                  "turns about the pinion's axis while that axis turns about the gear's; the "
                  "space's flanks are the envelope of the tooth's flank planes, which occt has no"
                  " operation to find")
