// The block under the bicubic Bézier dome of nurbs/bezier-dome, drilled through off centre.
//
// Refused: popkern's scripts make no surface from control points. A profile is lines and
// three-point arcs (`path`, `polygon`, `rect`, `circle`), and every face comes from extruding,
// revolving, sweeping or lofting one, so there is no way to give the dome's patch.
throw "popkern's scripts make no surface from control points: profiles are lines and arcs, and no \
feature takes a Bézier patch";
