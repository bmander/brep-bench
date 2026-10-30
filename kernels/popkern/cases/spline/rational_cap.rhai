// A ball's cap: the region in the xz plane between the z axis from -5 to 10, the floor z = -5
// out to 5√3, and the rational quadratic Bézier arc from (5√3, 0, -5) to (0, 0, 10), turned a
// full turn about z.
//
// popkern's scripts make no spline curve, but with its weights the Bézier arc is exactly the arc
// of the circle of radius 10 about the origin, as the case says, over its widest point
// (10, 0, 0), and a profile draws that arc exactly (u = x, v = z).
let m = model();
let r3 = sqrt(3.0);
m.profile(xz())
    .path(0.0, -5.0).line_to(5.0 * r3, -5.0)
    .arc_to([10.0, 0.0], [0.0, 10.0]).line_to(0.0, -5.0).close()
    .sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
