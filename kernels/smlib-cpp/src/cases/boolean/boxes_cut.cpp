// One block cut by another. Two blocks 10 high, both standing on the xy plane. The first spans x
// and y from 0 to 10; the second spans x and y from 5 to 15. Cut the second away from the first.
#include "bench.h"

static const double HEIGHT = 10;

CASE("boolean/boxes-cut")
{
    // A box is given by its lowest corner and its lengths along x, y and z.
    SmBrep *first = nullptr, *second = nullptr, *result = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), 10, 10, HEIGHT, first);
    CHECK(SmApiCreateBox, SmVector3d(5, 5, 0), 10, 10, HEIGHT, second);
    CHECK(SmApiBooleanDifference, first, second, result);
    return result;
}
