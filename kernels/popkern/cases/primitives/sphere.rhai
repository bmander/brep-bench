// popkern has no ball primitive: a half disc of radius 5 in the xz plane, its diameter on the
// z axis, revolved a full turn about z.
let m = model();
m.profile(xz())
    .path(0.0, -5.0).arc_to([5.0, 0.0], [0.0, 5.0]).line_to(0.0, -5.0).close()
    .sketch("half_disc")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
