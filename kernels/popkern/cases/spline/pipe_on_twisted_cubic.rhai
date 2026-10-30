// A pipe of radius 8 swept along a twisted (non-planar) cubic Bézier curve.
//
// Refused: popkern's scripts make no spline curve. A profile is lines and three-point arcs
// (`path`, `polygon`, `rect`, `circle`), and a sweep's path is an edge of a body those make, so
// there is no cubic Bézier for the pipe to follow.
throw "popkern's scripts make no spline curve: a pipe follows an edge of a body made from lines \
and arcs, and none is a cubic Bézier";
