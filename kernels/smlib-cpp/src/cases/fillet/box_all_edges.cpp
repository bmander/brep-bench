// A cube with every edge rounded. A cube of side 10 with all twelve of its edges rounded to radius
// 1. Three rounds meet at each corner.
#include "bench.h"

static const double SIDE = 10;
static const double RADIUS = 1;

CASE("fillet/box-all-edges")
{
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    CHECK(SmApiCircularFillet, cube, RADIUS);  // rounds every edge of the solid, in place
    return cube;
}
