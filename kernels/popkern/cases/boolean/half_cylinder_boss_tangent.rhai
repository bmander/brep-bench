// A cube of side 10 from the origin, united with a cylinder of radius 5 and height 10 on the
// axis x = 10, y = 5, whose wall touches the faces y = 0 and y = 10 along the cube's edges.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
let boss = m.tool(|m| m.profile(xy()).circle(5.0).at(10.0, 5.0)
    .sketch("boss").extrude(blind(10.0)));
cube = cube.union(boss);
m
