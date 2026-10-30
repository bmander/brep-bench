// A plate from (-10, -10, 0) to (10, 10, 2), drilled on the z axis with radius 3; both of the
// hole's rims, at z = 2 and z = 0, rounded to radius 1.
let m = model();
let plate = m.profile(xy()).rect(20.0, 20.0).sketch("plate").extrude(blind(2.0));
let drill = m.tool(|m| m.profile(xy_at(-1.0)).circle(3.0).sketch("drill").extrude(blind(4.0)));
plate = plate.subtract(drill);
plate = plate.fillet(edges().circular(), 1.0, #{ label: "rims" });
m
