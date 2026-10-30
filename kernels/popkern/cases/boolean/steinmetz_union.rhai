// Two cylinders of radius 5 and length 30 crossing at right angles, along x and along y, each
// extruded both ways from its middle, united.
let m = model();
let along_x = m.profile(yz()).circle(5.0).sketch("along_x").extrude(mid_plane(15.0));
let along_y = m.tool(|m| m.profile(xz()).circle(5.0).sketch("along_y").extrude(mid_plane(15.0)));
along_x = along_x.union(along_y);
m
