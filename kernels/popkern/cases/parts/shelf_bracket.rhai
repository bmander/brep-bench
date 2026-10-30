// An L bracket: its outline in the xz plane (u = x, v = z), the inside corner a quarter circle
// of radius 8 about (28, 92), extruded 5 each way along y.
let m = model();
let s = 4.0 * sqrt(2.0);
m.profile(xz())
    .path(0.0, 0.0).line_to(20.0, 0.0).line_to(20.0, 92.0)
    .arc_to([28.0 - s, 92.0 + s], [28.0, 100.0])
    .line_to(100.0, 100.0).line_to(100.0, 120.0).line_to(0.0, 120.0).line_to(0.0, 0.0).close()
    .sketch("outline").extrude(mid_plane(5.0));
m
