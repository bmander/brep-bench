// A rectangle 10 by 20 on the XY plane, its corner at the origin, extruded 30 up.
let m = model();
m.profile(xy()).rect(10.0, 20.0).at(5.0, 10.0).sketch("base").extrude(blind(30.0));
m
