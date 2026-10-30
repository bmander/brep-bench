// A block from the origin to (10, 4, 6); the two edges of its top running along x rounded to
// radius 2, half the top's width.
let m = model();
let slab = m.profile(xy()).rect(10.0, 4.0).at(5.0, 2.0).sketch("slab").extrude(blind(6.0));
slab = slab.fillet(edges().direction("x").at_height(6.0), 2.0, #{ label: "half_round" });
m
