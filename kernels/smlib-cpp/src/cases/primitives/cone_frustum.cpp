// A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and radius 2
// at its top (z = 6).
#include "bench.h"

static const SmVector3d BASE_CENTRE(0, 0, 0);
static const double BASE_RADIUS = 5, TOP_RADIUS = 2, HEIGHT = 6;

CASE("primitives/cone-frustum")
{
    // A cone with two non-zero radii is SMLib's frustum, standing on its base's centre along +z.
    SmBrep* frustum = nullptr;
    CHECK(SmApiCreateCone, BASE_CENTRE, BASE_RADIUS, TOP_RADIUS, HEIGHT, frustum);
    return frustum;
}
