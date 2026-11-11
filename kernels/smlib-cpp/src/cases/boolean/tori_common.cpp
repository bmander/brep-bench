// Two coaxial tori, their common part. Two solid tori about the z axis, each with a middle circle
// of radius 5 and a tube of radius 2. The first middle circle lies in z = −1, the second in z = 1.
// Make their common part.
#include "bench.h"

static const double MAJOR_RADIUS = 5, MINOR_RADIUS = 2;

CASE("boolean/tori-common")
{
    SmBrep *lower = nullptr, *upper = nullptr, *common = nullptr;
    CHECK(SmApiCreateTorus, SmVector3d(0, 0, -1), MAJOR_RADIUS, MINOR_RADIUS, lower);
    CHECK(SmApiCreateTorus, SmVector3d(0, 0, 1), MAJOR_RADIUS, MINOR_RADIUS, upper);
    CHECK(SmApiBooleanIntersection, lower, upper, common);
    return common;
}
