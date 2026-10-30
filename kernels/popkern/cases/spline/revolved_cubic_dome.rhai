// A dome turned a full turn about z from a region bounded by the z axis, the x axis and a cubic
// Bézier curve.
//
// Refused: popkern's scripts make no spline curve. A profile is lines and three-point arcs
// (`path`, `polygon`, `rect`, `circle`), and a sweep's path is an edge of a body those make, so
// the dome's cubic Bézier side cannot be drawn.
throw "popkern's scripts make no spline curve: a profile is lines and three-point arcs, so the \
dome's cubic Bézier side cannot be drawn";
