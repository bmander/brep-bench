// A block with a groove cut by a ball of radius 3 whose centre follows a cubic Bézier curve in
// the top face.
//
// Refused: popkern has no sweep of a solid, and a ball's sweep, the tube about its centre's path
// with a ball at each end, needs that path: popkern's scripts make no spline curve (a profile is
// lines and three-point arcs, and a pipe follows an edge of a body they make), so there is no
// cubic Bézier for the tube to follow.
throw "popkern has no sweep of a solid, and its scripts make no spline curve: there is no cubic \
Bézier path for the ball's tube to follow";
