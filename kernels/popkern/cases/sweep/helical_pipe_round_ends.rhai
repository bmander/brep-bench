// workaround: no sphere primitive
// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A circle of radius 1 swept along the helix of radius 10 about the z axis that rises 5 per turn,
// for three right-handed turns from (10, 0, 0) to (10, 0, 15); each flat end closed by a ball of
// radius 1 on the helix's end, which the pipe's end cuts in half. popkern's helix about +z
// starts on +y, its frame's first direction, and winds towards -x: started at -90° it starts on
// +x.
let m = model();
let coil = m.profile(xy()).circle(1.0).sketch("wire")
    .helical_sweep(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 10.0, 5.0, 3.0,
        #{ start_angle: -90.0, label: "coil" });
let start = m.tool(|m| ball(m, 1.0).translate(10.0, 0.0, 0.0));
coil = coil.union(start, #{ label: "start_end" });
let finish = m.tool(|m| ball(m, 1.0).translate(10.0, 0.0, 15.0));
coil = coil.union(finish, #{ label: "finish_end" });
m
