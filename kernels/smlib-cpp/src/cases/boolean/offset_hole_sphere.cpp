// A ball bored off its centre. A ball of radius 20 centred on the origin, with a round hole of
// radius 6 bored right through it parallel to the y axis. The hole's axis is the line x = 5, z = 0,
// so the hole stays inside the ball's outline. The cutting cylinder runs past the ball at both
// ends.
#include "bench.h"

static const double BALL_RADIUS = 20, HOLE_RADIUS = 6;
static const double HOLE_X = 5;  // the hole's axis is x = 5, z = 0

CASE("boolean/offset-hole-sphere")
{
    SmBrep *ball = nullptr, *drill = nullptr, *drilled = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), BALL_RADIUS, ball);
    // The drill is made along z, from z = −40 to z = 40, well past the ball at both ends; a
    // quarter turn back about the line through (5, 0, 0) along x takes +z to +y.
    CHECK(SmApiCreateCylinder, SmVector3d(HOLE_X, 0, -2 * BALL_RADIUS), HOLE_RADIUS,
          4 * BALL_RADIUS, drill);
    CHECK(SmApiRotate, drill, SmVector3d(HOLE_X, 0, 0), SmVector3d(1, 0, 0), -90);
    CHECK(SmApiBooleanDifference, ball, drill, drilled);
    return drilled;
}
