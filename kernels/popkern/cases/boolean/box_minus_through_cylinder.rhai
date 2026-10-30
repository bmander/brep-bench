// A cube of side 10 from the origin, drilled through along z by a cylinder of radius 3 at
// (5, 5) that runs past both faces, from z = -1 to z = 11.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
let drill = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).at(5.0, 5.0)
    .sketch("drill").extrude(blind(12.0)));
cube = cube.subtract(drill);
m
