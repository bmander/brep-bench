// A cube of side 10 from the origin; its edge where x = 10 meets y = 10 rounded with a radius
// growing linearly along it, 1 at z = 0 to 2 at z = 10.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
cube = cube.fillet(edges().direction("z").at_vertex([10.0, 10.0, 0.0]), 1.0,
    #{ law: linear(2.0), label: "growing" });
m
