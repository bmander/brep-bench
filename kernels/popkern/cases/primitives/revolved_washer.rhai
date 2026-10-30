// The rectangle x 6..10, z 0..4 in the xz plane, turned a full circle about z.
let m = model();
m.profile(xz()).rect(4.0, 4.0).at(8.0, 2.0).sketch("section")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
m
