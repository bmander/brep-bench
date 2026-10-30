// A stepped shaft: its section in the xz plane (u = x, v = z), the shoulder's inside corner a
// quarter circle of radius 4 about (16, 64), revolved a full turn about z.
let m = model();
let s = 2.0 * sqrt(2.0);
m.profile(xz())
    .path(0.0, 0.0).line_to(20.0, 0.0).line_to(20.0, 60.0).line_to(16.0, 60.0)
    .arc_to([16.0 - s, 64.0 - s], [12.0, 64.0])
    .line_to(12.0, 160.0).line_to(0.0, 160.0).line_to(0.0, 0.0).close()
    .sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
