// A Reuleaux tetrahedron. Four balls of radius 10, centred at the vertices of a regular tetrahedron
// of side 10: (0, 0, 0), (10, 0, 0), (5, 5√3, 0) and (5, 5√3/3, 10√(2/3)). Make the solid common to
// all four balls.
#include "bench.h"

#include <cmath>

static const double RADIUS = 10;

CASE("boolean/reuleaux-tetrahedron")
{
    const SmVector3d centres[] = {{0, 0, 0}, {10, 0, 0}, {5, 5 * std::sqrt(3.0), 0},
                                  {5, 5 * std::sqrt(3.0) / 3, 10 * std::sqrt(2.0 / 3)}};
    SmBrep* solid = nullptr;
    CHECK(SmApiCreateSphere, centres[0], RADIUS, solid);
    for (int i = 1; i < 4; ++i)
    {
        SmBrep* ball = nullptr;
        CHECK(SmApiCreateSphere, centres[i], RADIUS, ball);
        CHECK(SmApiBooleanIntersection, solid, ball, solid);
    }
    return solid;
}
