// A cube of side 10 from the origin, and one from (5, 0, 10) to (15, 10, 20) standing on half
// of its top, joined.
let m = model();
let first = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("first").extrude(blind(10.0));
let second = m.tool(|m| m.profile(xy_at(10.0)).rect(10.0, 10.0).at(10.0, 5.0)
    .sketch("second").extrude(blind(10.0)));
first = first.union(second);
m
