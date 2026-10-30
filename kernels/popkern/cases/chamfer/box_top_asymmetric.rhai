// A box from the origin to (20, 10, 6); the four edges round its top chamfered 1 across the top
// and 2 down the sides.
let m = model();
let box = m.profile(xy()).rect(20.0, 10.0).at(10.0, 5.0).sketch("box").extrude(blind(6.0));
box = box.chamfer(edges().at_height(6.0), 1.0,
    #{ kind: two_distance(faces().facing("z").at_height(6.0), 2.0), label: "bevel" });
m
