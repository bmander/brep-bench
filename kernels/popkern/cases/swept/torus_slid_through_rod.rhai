// A rod of radius 5 on the z axis from z = 0 to 20, grooved by a torus (axis parallel to y,
// centre circle 6, tube 2) slid without turning along z from centre (11.5, 0, -9) to
// (11.5, 0, 29).
//
// popkern has no sweep of a solid. Sliding fills the torus's hole (the slide, 38, is longer than
// the hole is wide), so the torus sweeps what the solid of its outline does: the convex solid of
// revolution about the torus's axis whose section is the tube's outer half-disc joined to the
// axis. That solid slid along z is itself at each end of the slide and, between, its section
// through the centre square to the slide (the stadium x = 11.5 ± 6 with half-discs of radius 2,
// |y| <= 2) carried along the slide. At its ends the torus is wholly below the rod (its top at
// z = -1) and wholly above it (its bottom at z = 21), so of everything it passes through only the
// stadium extruded from z = -9 to 29 meets the rod.
let m = model();
let rod = m.profile(xy()).circle(5.0).sketch("rod").extrude(blind(20.0));
let swept = m.tool(|m| m.profile(xy_at(-9.0))
    .path(5.5, -2.0).line_to(17.5, -2.0).arc_to([19.5, 0.0], [17.5, 2.0])
    .line_to(5.5, 2.0).arc_to([3.5, 0.0], [5.5, -2.0]).close()
    .sketch("stadium").extrude(blind(38.0)));
rod = rod.subtract(swept, #{ label: "groove" });
m
