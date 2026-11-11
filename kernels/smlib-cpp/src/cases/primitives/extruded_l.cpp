// An L-shaped plate. Its outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5),
// (5, 5), (5, 15) and (0, 15). Extrude it 8 along +z.
#include "bench.h"

static const std::vector<SmPoint3d> OUTLINE = {
    {0, 0, 0}, {20, 0, 0}, {20, 5, 0}, {5, 5, 0}, {5, 15, 0}, {0, 15, 0}};
static const double THICKNESS = 8;

CASE("primitives/extruded-l")
{
    // A closed planar loop swept along a line, capped at both ends, is a solid. The sweep copies
    // the curves; they stay the caller's.
    SmBrep* plate = nullptr;
    CHECK(SmApiCreateLinearSweep, bench::polygon(OUTLINE), SmVector3d(0, 0, 1), THICKNESS, TRUE,
          plate);
    return plate;
}
