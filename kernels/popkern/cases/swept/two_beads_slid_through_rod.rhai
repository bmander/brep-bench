// A rod of radius 5 on the z axis from z = 0 to 20, grooved by a wheel of two beads on one axis
// parallel to y, slid without turning 40 along z from below the rod to above it: the first bead
// centred at (11.5, 0, -9) with centre circle 6 and tube 2, the second at (11.5, 1.5, -9) with
// centre circle 6 and tube 1.5.
//
// popkern has no sweep of a solid. The wheel's sweep is the union of its beads' sweeps. Sliding
// fills each bead's hole (the slide, 40, is longer than any hole is wide), so a bead sweeps what
// the solid of its outline does: the convex solid of revolution about its axis whose section is
// the tube's outer half-disc joined to the axis. That solid slid along z is itself at each end
// of the slide and, between, its section through its centre square to the slide (a stadium:
// x = 11.5 ± 6 with half-discs of the tube's radius, across the tube in y) carried along the
// slide. At its ends the wheel is wholly below the rod (its top at z = -1) and wholly above it
// (its bottom at z = 23), so of everything it passes through only the two stadiums extruded from
// z = -9 to 31 meet the rod.
fn stadium(m, y, r, label) {
    m.profile(xy_at(-9.0))
        .path(5.5, y - r).line_to(17.5, y - r).arc_to([17.5 + r, y], [17.5, y + r])
        .line_to(5.5, y + r).arc_to([5.5 - r, y], [5.5, y - r]).close()
        .sketch(label).extrude(blind(40.0))
}

let m = model();
let rod = m.profile(xy()).circle(5.0).sketch("rod").extrude(blind(20.0));
let swept = m.tool(|m| {
    let first = stadium(m, 0.0, 2.0, "first_bead");
    let second = m.tool(|m| stadium(m, 1.5, 1.5, "second_bead"));
    first.union(second)
});
rod = rod.subtract(swept, #{ label: "groove" });
m
