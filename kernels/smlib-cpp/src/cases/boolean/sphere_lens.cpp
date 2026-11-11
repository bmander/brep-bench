// A lens. Two balls of radius 5, one centred on the origin and one at (6, 0, 0). Make the lens
// common to both.
#include "bench.h"

static const double RADIUS = 5;

CASE("boolean/sphere-lens")
{
    SmBrep *first = nullptr, *second = nullptr, *lens = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), RADIUS, first);
    CHECK(SmApiCreateSphere, SmVector3d(6, 0, 0), RADIUS, second);
    CHECK(SmApiBooleanIntersection, first, second, lens);
    return lens;
}
