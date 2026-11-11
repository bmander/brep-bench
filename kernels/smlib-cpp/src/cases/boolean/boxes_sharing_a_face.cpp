// Two cubes joined face to face. Two cubes of side 10, side by side: the first between the origin
// and (10, 10, 10), the second between (10, 0, 0) and (20, 10, 10). Join them into one solid. They
// touch along a whole face, which must disappear from the result.
#include "bench.h"

static const double SIDE = 10;

CASE("boolean/boxes-sharing-a-face")
{
    SmBrep *first = nullptr, *second = nullptr, *both = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, first);
    CHECK(SmApiCreateBox, SmVector3d(10, 0, 0), SIDE, SIDE, SIDE, second);
    CHECK(SmApiBooleanUnion, first, second, both);
    return both;
}
