// Three nested bodies that do not touch. Five balls: of radius 10 at the origin, 8 at (1, 0, 0), 6
// at (1, 1, 0), 4 at (1, 1, 1) and 2 at (1, 1.5, 1.5), each inside the one before. The solid is the
// first less the second, joined with the third less the fourth, joined with the fifth: a hollow
// ball holds a hollow ball, which holds a ball.
#include "bench.h"

static SmBrep* ball(const SmVector3d& centre, double radius)
{
    SmBrep* b = nullptr;
    CHECK(SmApiCreateSphere, centre, radius, b);
    return b;
}

CASE("boolean/nested-shells")
{
    SmBrep *outer = nullptr, *middle = nullptr, *all = nullptr;
    CHECK(SmApiBooleanDifference, ball({0, 0, 0}, 10), ball({1, 0, 0}, 8), outer);
    CHECK(SmApiBooleanDifference, ball({1, 1, 0}, 6), ball({1, 1, 1}, 4), middle);
    CHECK(SmApiBooleanUnion, outer, middle, all);
    CHECK(SmApiBooleanUnion, all, ball({1, 1.5, 1.5}, 2), all);
    return all;
}
