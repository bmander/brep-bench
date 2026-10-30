// A cylinder of radius 5 from z = 0 to 10, with one of radius 3 on the same axis and exactly the
// same ends cut from it.
let m = model();
let tube = m.profile(xy()).circle(5.0).sketch("outside").extrude(blind(10.0));
let core = m.tool(|m| m.profile(xy()).circle(3.0).sketch("core").extrude(blind(10.0)));
tube = tube.subtract(core);
m
