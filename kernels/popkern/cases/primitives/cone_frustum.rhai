// The frustum's half section in the xz plane (u = x, v = z), revolved a full turn about z.
let m = model();
m.profile(xz())
    .path(0.0, 0.0).line_to(5.0, 0.0).line_to(2.0, 6.0).line_to(0.0, 6.0).close()
    .sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
