// A disc of radius 20 from z = -5 to 0, with six holes of radius 2 on the circle of radius 12:
// the first at (12, 0), each other the first turned about z by a multiple of 60°, each its own
// cut by a cylinder running past both faces.
let m = model();
let disc = m.profile(xy_at(-5.0)).circle(20.0).sketch("disc").extrude(blind(5.0));
for k in 0..6 {
    let drill = m.tool(|m| m.profile(xy_at(-6.0)).circle(2.0).at(12.0, 0.0)
        .sketch(`drill_${k}`).extrude(blind(7.0)));
    if k > 0 {
        drill = drill.rotate(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), 60.0 * k.to_float());
    }
    disc = disc.subtract(drill, #{ label: `hole_${k}` });
}
m
