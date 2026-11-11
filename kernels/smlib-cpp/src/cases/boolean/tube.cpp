// A tube. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, with a hole of radius 3 bored
// along the axis. The cutting cylinder runs past both ends: from z = −1 to z = 11.
#include "bench.h"

static const double OUTER_RADIUS = 5, INNER_RADIUS = 3, LENGTH = 10;

CASE("boolean/tube")
{
    SmBrep *body = nullptr, *bore = nullptr, *tube = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), OUTER_RADIUS, LENGTH, body);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -1), INNER_RADIUS, LENGTH + 2, bore);
    CHECK(SmApiBooleanDifference, body, bore, tube);
    return tube;
}
