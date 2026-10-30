// workaround: no sphere primitive, no cone primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 15 on the origin, united with a cone on the z axis, apex (0, 0, -25), base the
// disc of radius 12 at z = -9: its section revolved about z. The cone's side is tangent to the
// sphere along the base's rim.
let m = model();
let ice_cream = ball(m, 15.0);
let cone = m.tool(|m| m.profile(xz())
    .path(0.0, -25.0).line_to(12.0, -9.0).line_to(0.0, -9.0).close()
    .sketch("cone_section").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle()));
ice_cream = ice_cream.union(cone);
m
