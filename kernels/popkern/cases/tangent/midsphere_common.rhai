// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A cube of side 10 on the origin, and the ball on the origin of radius 5√2 that touches its
// twelve edges: their common part.
let m = model();
let cube = m.profile(xy_at(-5.0)).rect(10.0, 10.0).sketch("cube").extrude(blind(10.0));
let midsphere = m.tool(|m| ball(m, 5.0 * sqrt(2.0)));
cube = cube.intersect(midsphere);
m
