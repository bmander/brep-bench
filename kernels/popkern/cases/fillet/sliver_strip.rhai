// A plate from the origin to (20, 10, 2); the two edges of its face y = 10 running along x
// rounded to radius 0.9999, leaving a strip 0.0002 tall between them.
let m = model();
let plate = m.profile(xy()).rect(20.0, 10.0).at(10.0, 5.0).sketch("plate").extrude(blind(2.0));
plate = plate.fillet(
    edges().direction("x").at_vertex([0.0, 10.0, 0.0])
        .or(edges().direction("x").at_vertex([0.0, 10.0, 2.0])),
    0.9999, #{ label: "rounds" });
m
