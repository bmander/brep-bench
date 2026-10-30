// A cube of side 10 from the origin, all twelve edges chamfered 2 on both faces.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
cube = cube.chamfer(edges(), 2.0, #{ label: "bevels" });
m
