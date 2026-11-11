// Two balls united. Two balls of radius 5, one centred on the origin and one at (6, 0, 0). Make
// their union.
#include "bench.h"

static const double RADIUS = 5;

CASE("boolean/spheres-union")
{
    SmBrep *first = nullptr, *second = nullptr, *both = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), RADIUS, first);
    CHECK(SmApiCreateSphere, SmVector3d(6, 0, 0), RADIUS, second);
    CHECK(SmApiBooleanUnion, first, second, both);
    return both;
}
