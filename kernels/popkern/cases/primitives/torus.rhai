// The tube's section, a circle of radius 2 centred 10 out along x in the xz plane, revolved a
// full turn about z.
// workaround: no torus primitive
let m = model();
m.profile(xz()).circle(2.0).at(10.0, 0.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
