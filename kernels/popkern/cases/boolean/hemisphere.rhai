// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A ball of radius 5 on the origin, with everything below the XY plane cut away by a block.
let m = model();
let dome = ball(m, 5.0);
let below = m.tool(|m| m.profile(xy_at(-6.0)).rect(12.0, 12.0).sketch("below").extrude(blind(6.0)));
dome = dome.subtract(below);
m
