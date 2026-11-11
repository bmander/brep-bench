// A napkin ring. A ball of radius 5 centred on the origin, with a hole of radius 3 bored right
// through it along the z axis.
#include "bench.h"

static const double BALL_RADIUS = 5, HOLE_RADIUS = 3;

CASE("boolean/napkin-ring")
{
    SmBrep *ball = nullptr, *drill = nullptr, *ring = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), BALL_RADIUS, ball);
    // The drill runs well past the ball at both ends: from z = −10 to z = 10.
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -2 * BALL_RADIUS), HOLE_RADIUS, 4 * BALL_RADIUS,
          drill);
    CHECK(SmApiBooleanDifference, ball, drill, ring);
    return ring;
}
