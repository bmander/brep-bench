// The common part of two barrel-vaulted blocks on the square |x|, |y| <= 12, the second the first
// turned a quarter turn about z.
//
// popkern's scripts make no surface from control points, but the vault's weighted quadratic arc
// is exactly the circle of radius 15 about (y, z) = (0, 1), as the case says, and a profile draws
// that arc exactly: the vault is its section in the yz plane (u = y, v = z), floor, sides and the
// arc from (12, 10) over its crown (0, 16) to (-12, 10), extruded both ways along x, so its top
// is popkern's exact cylinder.
fn vault(m, label) {
    m.profile(yz())
        .path(-12.0, 0.0).line_to(12.0, 0.0).line_to(12.0, 10.0)
        .arc_to([0.0, 16.0], [-12.0, 10.0]).line_to(-12.0, 0.0).close()
        .sketch(label).extrude(mid_plane(12.0))
}

let m = model();
let groin = vault(m, "along_x");
let crossing = m.tool(|m| vault(m, "along_y")
    .rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 90.0));
groin = groin.intersect(crossing);
m
