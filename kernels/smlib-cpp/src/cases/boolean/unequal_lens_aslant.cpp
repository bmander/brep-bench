// An unequal lens, aslant. Two balls: one of radius 5.511 centred on the origin, and one of radius
// 3.317 centred at (4.231, 3.467, 2.653), so that they overlap but neither holds the other. Make
// the lens common to both.
#include "bench.h"

CASE("boolean/unequal-lens-aslant")
{
    SmBrep *first = nullptr, *second = nullptr, *lens = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), 5.511, first);
    CHECK(SmApiCreateSphere, SmVector3d(4.231, 3.467, 2.653), 3.317, second);
    CHECK(SmApiBooleanIntersection, first, second, lens);
    return lens;
}
