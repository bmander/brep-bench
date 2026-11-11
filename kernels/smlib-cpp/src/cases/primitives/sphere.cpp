// A ball: a solid ball of radius 5 centred on the origin.
#include "bench.h"

static const SmVector3d CENTRE(0, 0, 0);
static const double RADIUS = 5;

CASE("primitives/sphere")
{
    SmBrep* ball = nullptr;
    CHECK(SmApiCreateSphere, CENTRE, RADIUS, ball);
    return ball;
}
