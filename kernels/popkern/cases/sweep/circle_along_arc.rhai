// A pipe of radius 2 along the quarter arc of radius 10 about the z axis, from (10, 0, 0) to
// (0, 10, 0). A pipe follows an edge of a body: the path's body is the quarter disc of radius 10
// extruded 1 up, whose arc at z = 0 is the path. The pipe is a new body, the model's last, and the
// quarter disc only carries its path.
let m = model();
let h = 10.0 / sqrt(2.0);
let rail = m.profile(xy())
    .path(0.0, 0.0).line_to(10.0, 0.0).arc_to([h, h], [0.0, 10.0]).line_to(0.0, 0.0).close()
    .sketch("rail_section").extrude(blind(1.0));
rail.pipe(edges().at_height(0.0).chord_length_between(14.0, 14.3), 2.0, #{ label: "pipe" });
m
