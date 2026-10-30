// A spheroid turned from a weighted quadratic B-spline, each of its two segments a quarter of
// the ellipse x²/36 + z²/100 = 1.
//
// Refused: popkern's scripts make no spline curve. A profile is lines and three-point arcs
// (`path`, `polygon`, `rect`, `circle`), and a sweep's path is an edge of a body those make, so
// neither the weighted B-spline nor the ellipse it traces can be drawn (a profile's arcs are
// circular).
throw "popkern's scripts make no spline curve and no ellipse: a profile is lines and circular \
arcs, so the spheroid's weighted B-spline (quarter ellipses) cannot be drawn";
