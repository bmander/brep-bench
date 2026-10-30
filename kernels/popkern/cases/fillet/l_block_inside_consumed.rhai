// The L section in the xz plane, extruded from y = 0 to 10 (xz_at(-10) is the plane y = 10, and
// the XZ plane extrudes towards -y); its inside edge, the line x = 3, z = 3, rounded to radius 7.
let m = model();
let block = m.profile(xz_at(-10.0))
    .polygon([[0.0, 0.0], [10.0, 0.0], [10.0, 3.0], [3.0, 3.0], [3.0, 10.0], [0.0, 10.0]])
    .sketch("section").extrude(blind(10.0));
block = block.fillet(edges().direction("y").at_vertex([3.0, 0.0, 3.0]), 7.0, #{ label: "inside" });
m
