// popkern has no ball primitive: a half disc revolved about its diameter on the z axis.
fn ball(m, r) {
    m.profile(xz()).path(0.0, -r).arc_to([r, 0.0], [0.0, r]).line_to(0.0, -r).close()
        .sketch("half_disc").revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle())
}

// A block from (-6, -6, -8) to (18, 18, 0), with a groove cut by a ball of radius 3 whose centre
// swings a quarter turn about the z axis from (12, 0, 0) to (0, 12, 0).
//
// popkern has no sweep of a solid; a ball's sweep is the tube about its centre's path with a ball
// at each end. The tube is the quarter of the torus about z (centre circle 12, tube 3) between
// the planes x = 0 and y = 0: the whole torus revolved and cut down to the quadrant, since a
// revolve through part of a turn makes NURBS faces where a full turn makes the exact torus. The
// tube and the balls are cut from the block one after another.
let m = model();
let block = m.profile(xy_at(-8.0)).rect(24.0, 24.0).at(6.0, 6.0)
    .sketch("block").extrude(blind(8.0));
let tube = m.tool(|m| {
    let torus = m.profile(xz()).circle(3.0).at(12.0, 0.0).sketch("tube")
        .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), full_circle());
    let quadrant = m.tool(|m| m.profile(xy_at(-5.0)).rect(20.0, 20.0).at(10.0, 10.0)
        .sketch("quadrant").extrude(blind(10.0)));
    torus.intersect(quadrant)
});
block = block.subtract(tube, #{ label: "channel" });
let start = m.tool(|m| ball(m, 3.0).translate(12.0, 0.0, 0.0));
block = block.subtract(start, #{ label: "start_end" });
let finish = m.tool(|m| ball(m, 3.0).translate(0.0, 12.0, 0.0));
block = block.subtract(finish, #{ label: "finish_end" });
m
