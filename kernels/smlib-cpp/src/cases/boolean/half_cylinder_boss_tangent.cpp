// A cube with a half-round end. Take a cube of side 10 between the origin and (10, 10, 10). Unite
// it with a cylinder of radius 5 and height 10 that stands on the xy plane, its axis at x = 10, y =
// 5. Half of the cylinder lies inside the cube. Its surface touches the cube's faces y = 0 and y =
// 10 exactly along the cube's edges at x = 10.
#include "bench.h"

static const double SIDE = 10;
static const double BOSS_RADIUS = 5, BOSS_HEIGHT = 10;

CASE("boolean/half-cylinder-boss-tangent")
{
    SmBrep *cube = nullptr, *boss = nullptr, *both = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    CHECK(SmApiCreateCylinder, SmVector3d(10, 5, 0), BOSS_RADIUS, BOSS_HEIGHT, boss);
    CHECK(SmApiBooleanUnion, cube, boss, both);
    return both;
}
