// A cylinder of radius 5 from z = 0 to 10, the rim of its top chamfered 5 on both faces, its own
// radius, so the top shrinks to a point.
let m = model();
let drum = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
drum = drum.chamfer(edges().circular().at_height(10.0), 5.0, #{ label: "point" });
m
