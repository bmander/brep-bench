// A stepped shaft on the z axis, made one solid. The first cylinder has radius 5 and runs from z =
// 0 to z = 10. The second has radius 3 and runs from z = 10 to z = 20, standing on top of the
// first.
#include "bench.h"

static const double LOWER_RADIUS = 5, UPPER_RADIUS = 3;
static const double STEP_HEIGHT = 10;  // each cylinder's length, and where the second stands

CASE("boolean/stepped-shaft")
{
    SmBrep *lower = nullptr, *upper = nullptr, *shaft = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), LOWER_RADIUS, STEP_HEIGHT, lower);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, STEP_HEIGHT), UPPER_RADIUS, STEP_HEIGHT, upper);
    CHECK(SmApiBooleanUnion, lower, upper, shaft);
    return shaft;
}
