// Two disjoint but linked tori, both of major radius 3 and tube radius 0.5: one at the origin about
// z, one at (3, 0, 0) about y. Their union keeps two separate solids.
#include "bench.h"

static const double MAJOR_RADIUS = 3, MINOR_RADIUS = 0.5;
static const SmVector3d SECOND_CENTRE(3, 0, 0);

CASE("boolean/linked-tori")
{
    SmBrep *first = nullptr, *second = nullptr, *both = nullptr;
    CHECK(SmApiCreateTorus, SmVector3d(0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS, first);
    // A torus is made about z; a quarter turn about the line through its centre along x lays its
    // middle circle in the plane y = 0, about a line parallel to y.
    CHECK(SmApiCreateTorus, SECOND_CENTRE, MAJOR_RADIUS, MINOR_RADIUS, second);
    CHECK(SmApiRotate, second, SECOND_CENTRE, SmVector3d(1, 0, 0), 90);
    CHECK(SmApiBooleanUnion, first, second, both);
    return both;
}
