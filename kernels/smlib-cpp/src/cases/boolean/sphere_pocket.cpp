// A block with a hemispherical pocket. A block from (−10, −10, −10) to (10, 10, 0). Cut from it a
// ball of radius 5 centred on the origin, leaving a hemispherical pocket in its top face.
#include "bench.h"

static const double BALL_RADIUS = 5;

CASE("boolean/sphere-pocket")
{
    SmBrep *block = nullptr, *ball = nullptr, *pocketed = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-10, -10, -10), 20, 20, 10, block);
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), BALL_RADIUS, ball);
    CHECK(SmApiBooleanDifference, block, ball, pocketed);
    return pocketed;
}
