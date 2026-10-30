// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 20 on the origin, bored parallel to y by a cylinder of radius 6 on the line
// x = 5, z = 0, running past the ball at both ends.
let m = model();
let pierced = ball(m, 20.0);
let bore = m.tool(|m| m.profile(xz()).circle(6.0).at(5.0, 0.0)
    .sketch("bore").extrude(mid_plane(21.0)));
pierced = pierced.subtract(bore);
m
