// A cylinder: radius 5 and height 10, its axis the z axis, standing on the xy plane (it runs from z
// = 0 to z = 10).
#include "bench.h"

static const SmVector3d BASE_CENTRE(0, 0, 0);
static const double RADIUS = 5, HEIGHT = 10;

CASE("primitives/cylinder")
{
    // The cylinder stands on its base's centre along +z.
    SmBrep* cylinder = nullptr;
    CHECK(SmApiCreateCylinder, BASE_CENTRE, RADIUS, HEIGHT, cylinder);
    return cylinder;
}
