// The L outline on the XY plane, extruded 8 up.
let m = model();
m.profile(xy())
    .polygon([[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]])
    .sketch("outline")
    .extrude(blind(8.0));
m
