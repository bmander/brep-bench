// Half a torus. A torus about the z axis, centred on the origin, whose tube's centre circle has
// radius 10 and whose tube has radius 2. Cut away everything where x < 0.
#include "bench.h"

static const double MAJOR_RADIUS = 10, MINOR_RADIUS = 2;

CASE("boolean/half-torus")
{
    SmBrep* torus = nullptr;
    CHECK(SmApiCreateTorus, SmVector3d(0, 0, 0), MAJOR_RADIUS, MINOR_RADIUS, torus);
    // SMLib's cut by a plane (SmApiCut) trims the faces and leaves the body open, not a solid. So
    // "everything where x < 0" is cut away as a block: one with a face in the plane x = 0 that
    // holds all of the torus on that side, with a unit to spare.
    const double reach = MAJOR_RADIUS + MINOR_RADIUS + 1;
    SmBrep* negative_x = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-reach, -reach, -MINOR_RADIUS - 1), reach, 2 * reach,
          2 * MINOR_RADIUS + 2, negative_x);
    CHECK(SmApiBooleanDifference, torus, negative_x, torus);
    return torus;
}
