// A cube drilled through on a grid three ways. A cube from the origin to (30, 30, 30), drilled by
// fifteen holes of radius 2.5: three along x at y = 7.5, 15, 22.5 and z = 15, three along y at x =
// 7.5, 15, 22.5 and z = 15, and nine along z through (x, y) for x and y each 7.5, 15, 22.5.
#include "bench.h"

static const double SIDE = 30;
static const double RADIUS = 2.5;
static const double GRID[] = {7.5, 15, 22.5};
static const double MIDDLE = 15;

/// A drill whose axis passes through `through`, running a unit past the cube at both ends. It is
/// made along z, and turned `angle` degrees about `axis` through that point to lie along x or y.
static SmBrep* drill(const SmPoint3d& through, const SmVector3d& axis = SmVector3d(0, 0, 1),
                     double angle = 0)
{
    SmBrep* tool = nullptr;
    CHECK(SmApiCreateCylinder, through - SmVector3d(0, 0, MIDDLE + 1), RADIUS, SIDE + 2, tool);
    if (angle != 0)
        CHECK(SmApiRotate, tool, through, axis, angle);
    return tool;
}

CASE("boolean/cross-drilled-cube")
{
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    const SmVector3d ALONG_Y_AXIS(0, 1, 0), ALONG_X_AXIS(1, 0, 0);
    std::vector<SmBrep*> drills;
    for (double g : GRID)  // about +y a quarter turn takes +z to +x
        drills.push_back(drill(SmPoint3d(MIDDLE, g, MIDDLE), ALONG_Y_AXIS, 90));
    for (double g : GRID)  // about +x a quarter turn back takes +z to +y
        drills.push_back(drill(SmPoint3d(g, MIDDLE, MIDDLE), ALONG_X_AXIS, -90));
    for (double x : GRID)
        for (double y : GRID)
            drills.push_back(drill(SmPoint3d(x, y, MIDDLE)));
    for (SmBrep* tool : drills)
        CHECK(SmApiBooleanDifference, cube, tool, cube);
    return cube;
}
