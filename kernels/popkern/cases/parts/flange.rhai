// A flange on the z axis: a plate of radius 32 from z = 0 to 6 and a hub of radius 15 on it up to
// z = 16, bored with radius 8 right through; six bolt holes of radius 3 on the circle of radius
// 24, one every 60°, each cut by a cylinder running past both faces of the plate.
let m = model();
let flange = m.profile(xy()).circle(32.0).sketch("plate").extrude(blind(6.0));
let hub = m.tool(|m| m.profile(xy_at(6.0)).circle(15.0).sketch("hub").extrude(blind(10.0)));
flange = flange.union(hub);
let bore = m.tool(|m| m.profile(xy_at(-1.0)).circle(8.0).sketch("bore").extrude(blind(18.0)));
flange = flange.subtract(bore);
for k in 0..6 {
    let bolt = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).at(24.0, 0.0)
        .sketch(`bolt_${k}`).extrude(blind(8.0)));
    if k > 0 {
        bolt = bolt.rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 60.0 * k.to_float());
    }
    flange = flange.subtract(bolt, #{ label: `bolt_hole_${k}` });
}
m
