// A pulley about z: a hub of radius 12 from z = -14 to 14 and a rim of radius 28 from z = -9 to 9,
// joined; bored with radius 6 by a cylinder running past both ends; and a V groove, the triangle
// (22, 0), (29, -5), (29, 5) of the xz plane (u = x, v = z) revolved a full turn and taken away.
let m = model();
let z = axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
let pulley = m.profile(xy_at(-14.0)).circle(12.0).sketch("hub").extrude(blind(28.0));
let rim = m.tool(|m| m.profile(xy_at(-9.0)).circle(28.0).sketch("rim").extrude(blind(18.0)));
pulley = pulley.union(rim);
let bore = m.tool(|m| m.profile(xy_at(-15.0)).circle(6.0).sketch("bore").extrude(blind(30.0)));
pulley = pulley.subtract(bore);
let groove = m.tool(|m| m.profile(xz()).polygon([[22.0, 0.0], [29.0, -5.0], [29.0, 5.0]])
    .sketch("groove").revolve(z, full_circle()));
pulley = pulley.subtract(groove);
m
