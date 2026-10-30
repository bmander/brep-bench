// A barrel-vaulted block on the square |x|, |y| <= 12, with everything above its crown, z = 16,
// cut away by a box whose floor is that plane: the floor touches the vault along its crown.
//
// popkern's scripts make no surface from control points, but the vault's weighted quadratic arc
// is exactly the circle of radius 15 about (y, z) = (0, 1), as the case says, and a profile draws
// that arc exactly: the vault is its section in the yz plane (u = y, v = z), floor, sides and the
// arc from (12, 10) over its crown (0, 16) to (-12, 10), extruded both ways along x, so its top
// is popkern's exact cylinder.
let m = model();
let block = m.profile(yz())
    .path(-12.0, 0.0).line_to(12.0, 0.0).line_to(12.0, 10.0)
    .arc_to([0.0, 16.0], [-12.0, 10.0]).line_to(-12.0, 0.0).close()
    .sketch("vault").extrude(mid_plane(12.0));
let above = m.tool(|m| m.profile(xy_at(16.0)).rect(30.0, 30.0)
    .sketch("above").extrude(blind(10.0)));
block = block.subtract(above);
m
