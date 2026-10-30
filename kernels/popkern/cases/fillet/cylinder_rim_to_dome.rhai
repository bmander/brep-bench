// A cylinder of radius 5 from z = 0 to 10, the rim of its top rounded to radius 5, its own
// radius.
let m = model();
let drum = m.profile(xy()).circle(5.0).sketch("cylinder").extrude(blind(10.0));
drum = drum.fillet(edges().circular().at_height(10.0), 5.0, #{ label: "dome" });
m
