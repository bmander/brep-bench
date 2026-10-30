// A cylinder of radius 5 from z = 0 to 10, and one of radius 3 from z = 10 to 20 on top, joined.
let m = model();
let shaft = m.profile(xy()).circle(5.0).sketch("thick").extrude(blind(10.0));
let thin = m.tool(|m| m.profile(xy_at(10.0)).circle(3.0).sketch("thin").extrude(blind(10.0)));
shaft = shaft.union(thin);
m
