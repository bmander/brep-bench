// workaround: no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A cube of side 10 from the origin, united with its inscribed ball, radius 5 at (5, 5, 5).
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
let inscribed = m.tool(|m| ball(m, 5.0).translate(5.0, 5.0, 5.0));
cube = cube.union(inscribed);
m
