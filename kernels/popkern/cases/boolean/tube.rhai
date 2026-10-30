// A cylinder of radius 5 from z = 0 to 10, bored along its axis by a cylinder of radius 3 that
// runs past both ends, from z = -1 to 11.
let m = model();
let tube = m.profile(xy()).circle(5.0).sketch("outside").extrude(blind(10.0));
let bore = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).sketch("bore").extrude(blind(12.0)));
tube = tube.subtract(bore);
m
