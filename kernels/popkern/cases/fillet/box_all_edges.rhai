// A cube of side 10 from the origin, all twelve edges rounded to radius 1.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
cube = cube.fillet(edges(), 1.0, #{ label: "rounds" });
m
