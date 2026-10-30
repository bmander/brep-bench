// A torus about z (centre circle 10, tube 2), with everything where x < 0 cut away by a block
// that reaches past it on every other side.
let m = model();
let torus = m.profile(xz()).circle(2.0).at(10.0, 0.0).sketch("tube")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
let half = m.tool(|m| m.profile(xy_at(-5.0)).rect(15.0, 30.0).at(-7.5, 0.0)
    .sketch("half").extrude(blind(10.0)));
torus = torus.subtract(half);
m
