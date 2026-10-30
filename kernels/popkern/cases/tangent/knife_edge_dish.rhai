// workaround: no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A cylinder of radius 5 on the z axis from z = 0 to 10, with a ball of radius 5 at (0, 0, 10),
// the middle of its top, cut from it.
let m = model();
let dish = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
let scoop = m.tool(|m| ball(m, 5.0).translate(0.0, 0.0, 10.0));
dish = dish.subtract(scoop);
m
