// A plate from the origin to (100, 20, 10), drilled nine times along z at y = 10, x = 10 .. 90:
// each hole its own cut, by a cylinder of radius 2 running past both faces.
let m = model();
let plate = m.profile(xy()).rect(100.0, 20.0).at(50.0, 10.0).sketch("plate").extrude(blind(10.0));
for i in 1..10 {
    let x = 10.0 * i.to_float();
    let drill = m.tool(|m| m.profile(xy_at(-1.0)).circle(2.0).at(x, 10.0)
        .sketch(`drill_${i}`).extrude(blind(12.0)));
    plate = plate.subtract(drill, #{ label: `hole_${i}` });
}
m
