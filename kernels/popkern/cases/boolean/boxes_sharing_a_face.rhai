// Two cubes of side 10 side by side along x, touching along the face x = 10, joined.
let m = model();
let first = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("first").extrude(blind(10.0));
let second = m.tool(|m| m.profile(xy()).rect(10.0, 10.0).at(15.0, 5.0)
    .sketch("second").extrude(blind(10.0)));
first = first.union(second);
m
