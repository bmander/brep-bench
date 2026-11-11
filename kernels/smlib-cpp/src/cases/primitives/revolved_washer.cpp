// A washer. In the xz plane, take the rectangle with x from 6 to 10 and z from 0 to 4. Revolve it a
// full turn about the z axis.
#include "bench.h"

static const double INNER = 6, OUTER = 10;  // x
static const double BOTTOM = 0, TOP = 4;    // z

CASE("primitives/revolved-washer")
{
    auto rectangle = bench::polygon(
        {{INNER, 0, BOTTOM}, {OUTER, 0, BOTTOM}, {OUTER, 0, TOP}, {INNER, 0, TOP}});
    // About the line through the origin along +z; a turn of 360° closes the solid on itself.
    SmBrep* washer = nullptr;
    CHECK(SmApiCreateRotationalSweep, rectangle, SmPoint3d(0, 0, 0), SmVector3d(0, 0, 1), 360,
          TRUE, washer);
    return washer;
}
