// The octahedron |x| + |y| + |z| ≤ 10 that the chamfer and fillet cases on it start from.
#pragma once

#include "bench.h"

#include <cmath>

namespace bench
{
/// The octahedron whose six corners are on the axes, `reach` from the origin: two square
/// pyramids base to base. SMLib's pyramid stands on a square whose sides are square to the x and y
/// axes, its apex over the square's centre: an eighth of a turn puts its corners on the axes.
inline SmBrep* octahedron(double reach)
{
    SmBrep *upper = nullptr, *lower = nullptr, *both = nullptr;
    CHECK(SmApiCreatePyramid, SmVector3d(0, 0, 0), reach * std::sqrt(2.0), reach, upper);
    CHECK(SmApiRotate, upper, SmVector3d(0, 0, 0), SmVector3d(0, 0, 1), 45);
    CHECK(SmApiCreatePyramid, SmVector3d(0, 0, 0), reach * std::sqrt(2.0), reach, lower);
    CHECK(SmApiRotate, lower, SmVector3d(0, 0, 0), SmVector3d(0, 0, 1), 45);
    CHECK(SmApiRotate, lower, SmVector3d(0, 0, 0), SmVector3d(1, 0, 0), 180);
    CHECK(SmApiBooleanUnion, upper, lower, both);
    expect_count(edges_of(both).GetSize(), 12, "edges");
    return both;
}
} // namespace bench
