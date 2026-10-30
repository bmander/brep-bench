// A circle of radius 5 about the z axis, extruded from z = 0 up to z = 10.
let m = model();
m.profile(xy()).circle(5.0).sketch("base").extrude(blind(10.0));
m
