// A bar of square section 20 × 20 swept along a planar cubic Bézier curve.
//
// Refused: popkern's scripts make no spline curve. A profile is lines and three-point arcs
// (`path`, `polygon`, `rect`, `circle`), and a sweep's path is an edge of a body those make, so
// there is no cubic Bézier for the bar to follow.
throw "popkern's scripts make no spline curve: a sweep follows an edge of a body made from lines \
and arcs, and none is a cubic Bézier";
