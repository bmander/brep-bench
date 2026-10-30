// Two blocks 10 high on the XY plane, x and y 0..10 and 5..15, united.
let m = model();
let first = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("first").extrude(blind(10.0));
let second = m.tool(|m| m.profile(xy()).rect(10.0, 10.0).at(10.0, 10.0)
    .sketch("second").extrude(blind(10.0)));
first = first.union(second);
m
