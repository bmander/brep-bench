// A block from the origin to (10, 4, 6); the two edges of its top running along x chamfered 2
// on both faces, half the top's width, so the chamfers meet in a ridge.
let m = model();
let block = m.profile(xy()).rect(10.0, 4.0).at(5.0, 2.0).sketch("block").extrude(blind(6.0));
block = block.chamfer(edges().direction("x").at_height(6.0), 2.0, #{ label: "ridge" });
m
