// A block on the square 0..20 by 0..20 under a bicubic Bézier patch of sixteen control points.
//
// Refused: popkern's scripts make no surface from control points. A profile is lines and
// three-point arcs (`path`, `polygon`, `rect`, `circle`), and every face comes from extruding,
// revolving, sweeping or lofting one, so there is no way to give the dome's patch.
throw "popkern's scripts make no surface from control points: profiles are lines and arcs, and no \
feature takes a Bézier patch";
