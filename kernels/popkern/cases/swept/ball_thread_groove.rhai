// workaround: no sweep of a solid, no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A rod of radius 5 on the z axis from z = 0 to 20, grooved by a ball of radius 1 whose centre
// follows the helix of radius 5 that rises 4 per turn, six right-handed turns from (5, 0, -2) to
// (5, 0, 22).
//
// popkern has no sweep of a solid; a ball's sweep is the tube about its centre's path with a ball
// at each end: a circle of radius 1 swept along the helix, square to it, and the two balls.
// popkern's helix about +z starts on +y, its frame's first direction, and winds towards -x:
// started at -90° it starts on +x.
let m = model();
let rod = m.profile(xy()).circle(5.0).sketch("rod").extrude(blind(20.0));
let swept = m.tool(|m| {
    let tube = m.profile(xy()).circle(1.0).sketch("ball_section")
        .helical_sweep(axis_at([0.0, 0.0, -2.0], [0.0, 0.0, 1.0]), 5.0, 4.0, 6.0,
            #{ start_angle: -90.0, label: "track" });
    let start = m.tool(|m| ball(m, 1.0).translate(5.0, 0.0, -2.0));
    tube = tube.union(start);
    let finish = m.tool(|m| ball(m, 1.0).translate(5.0, 0.0, 22.0));
    tube.union(finish)
});
rod = rod.subtract(swept, #{ label: "groove" });
m
