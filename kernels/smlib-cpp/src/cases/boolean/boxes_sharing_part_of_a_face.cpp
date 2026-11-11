// A cube on a cube, overhanging. Two cubes of side 10, joined into one solid. The first lies
// between the origin and (10, 10, 10). The second lies between (5, 0, 10) and (15, 10, 20): it
// stands on the first, and its bottom covers half of the first's top.
#include "bench.h"

static const double SIDE = 10;

CASE("boolean/boxes-sharing-part-of-a-face")
{
    SmBrep *lower = nullptr, *upper = nullptr, *both = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, lower);
    CHECK(SmApiCreateBox, SmVector3d(5, 0, 10), SIDE, SIDE, SIDE, upper);
    CHECK(SmApiBooleanUnion, lower, upper, both);
    return both;
}
