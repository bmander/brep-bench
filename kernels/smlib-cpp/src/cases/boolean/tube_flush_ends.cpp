// A tube bored flush. A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Cut from it a
// cylinder of radius 3 on the same axis that also runs exactly from z = 0 to z = 10. The ends of
// the cutting cylinder lie in the ends of the first.
#include "bench.h"

static const double OUTER_RADIUS = 5, INNER_RADIUS = 3, LENGTH = 10;

CASE("boolean/tube-flush-ends")
{
    // Both cylinders stand on the origin along +z, so their ends are coplanar.
    SmBrep *body = nullptr, *bore = nullptr, *tube = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), OUTER_RADIUS, LENGTH, body);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), INNER_RADIUS, LENGTH, bore);
    CHECK(SmApiBooleanDifference, body, bore, tube);
    return tube;
}
