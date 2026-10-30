// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// Two balls of radius 5, on the origin and on (6, 0, 0): their common part.
let m = model();
let lens = ball(m, 5.0);
let other = m.tool(|m| ball(m, 5.0).translate(6.0, 0.0, 0.0));
lens = lens.intersect(other);
m
