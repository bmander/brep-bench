// A quarter of a washer. In the xz plane, take the rectangle with x from 6 to 10 and z from 0 to 4.
// Revolve it 90° about the z axis, right-handed: from +x towards +y.
#include "bench.h"

static const double INNER = 6, OUTER = 10;  // x
static const double BOTTOM = 0, TOP = 4;    // z
static const double ANGLE = 90;             // degrees

CASE("primitives/revolved-sector")
{
    auto rectangle = bench::polygon(
        {{INNER, 0, BOTTOM}, {OUTER, 0, BOTTOM}, {OUTER, 0, TOP}, {INNER, 0, TOP}});
    // About the line through the origin along +z; the ends of a partial turn are capped.
    SmBrep* sector = nullptr;
    CHECK(SmApiCreateRotationalSweep, rectangle, SmPoint3d(0, 0, 0), SmVector3d(0, 0, 1), ANGLE,
          TRUE, sector);
    return sector;
}
