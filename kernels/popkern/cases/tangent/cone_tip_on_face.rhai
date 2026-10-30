// A block from (-10, -10, 0) to (10, 10, 10), united with a cone on the z axis whose apex is the
// origin and whose base is the disc of radius 5 at z = 15: its section revolved about z.
// workaround: no cone primitive
let m = model();
let block = m.profile(xy()).rect(20.0, 20.0).sketch("block").extrude(blind(10.0));
let cone = m.tool(|m| m.profile(xz())
    .path(0.0, 0.0).line_to(5.0, 15.0).line_to(0.0, 15.0).close()
    .sketch("cone_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()));
block = block.union(cone);
m
