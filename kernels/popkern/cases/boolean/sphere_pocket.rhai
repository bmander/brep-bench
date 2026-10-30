// workaround: no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A block from (-10, -10, -10) to (10, 10, 0), with a ball of radius 5 on the origin cut from it.
let m = model();
let block = m.profile(xy_at(-10.0)).rect(20.0, 20.0).sketch("block").extrude(blind(10.0));
let pocket = m.tool(|m| ball(m, 5.0));
block = block.subtract(pocket);
m
