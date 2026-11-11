// A cube drilled through. Take a cube of side 10 between the origin and (10, 10, 10). Drill a hole
// of radius 3 straight through it along z, centred at x = 5, y = 5. The cutting cylinder runs past
// both faces: from z = −1 to z = 11.
#include "bench.h"

static const double SIDE = 10;
static const double HOLE_RADIUS = 3;
static const double HOLE_FROM = -1, HOLE_TO = 11;  // z, past both faces

CASE("boolean/box-minus-through-cylinder")
{
    SmBrep *cube = nullptr, *drill = nullptr, *drilled = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    // A cylinder stands on its base's centre along +z.
    CHECK(SmApiCreateCylinder, SmVector3d(5, 5, HOLE_FROM), HOLE_RADIUS, HOLE_TO - HOLE_FROM,
          drill);
    // A Boolean changes its first operand in place and returns it, and deletes the second.
    CHECK(SmApiBooleanDifference, cube, drill, drilled);
    return drilled;
}
