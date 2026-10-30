// workaround: no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 5 on the origin, bored through along z by a cylinder of radius 3.
let m = model();
let ring = ball(m, 5.0);
let bore = m.tool(|m| m.profile(xy_at(-6.0)).circle(3.0).sketch("bore").extrude(blind(12.0)));
ring = ring.subtract(bore);
m
