// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 5 on the origin, united with a cylinder of the same radius on the z axis from
// z = -3 to 3, which it touches from inside along the equator.
let m = model();
let banded = ball(m, 5.0);
let band = m.tool(|m| m.profile(xy_at(-3.0)).circle(5.0).sketch("band").extrude(blind(6.0)));
banded = banded.union(band);
m
