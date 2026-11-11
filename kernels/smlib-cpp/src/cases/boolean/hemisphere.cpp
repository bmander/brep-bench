// A hemisphere. A ball of radius 5 centred on the origin, with everything below the xy plane cut
// away.
#include "bench.h"

static const double RADIUS = 5;

CASE("boolean/hemisphere")
{
    SmBrep* ball = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), RADIUS, ball);
    // SMLib's cut by a plane (SmApiCut) trims the faces and leaves the body open, not a solid. So
    // everything below the xy plane is cut away as a block: one with its top in that plane that
    // holds the lower half of the ball, with a unit to spare.
    const double reach = RADIUS + 1;
    SmBrep* below = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-reach, -reach, -reach), 2 * reach, 2 * reach, reach, below);
    CHECK(SmApiBooleanDifference, ball, below, ball);
    return ball;
}
