// A torus: a solid torus centred on the origin, about the z axis. The circle through the middle of
// its tube has radius 10, and the tube has radius 2.
#include "bench.h"

static const SmVector3d CENTRE(0, 0, 0);
static const double MAJOR_RADIUS = 10;  // the circle through the middle of the tube
static const double MINOR_RADIUS = 2;   // the tube

CASE("primitives/torus")
{
    // SMLib's torus has its middle circle in the xy plane: it is about z.
    SmBrep* torus = nullptr;
    CHECK(SmApiCreateTorus, CENTRE, MAJOR_RADIUS, MINOR_RADIUS, torus);
    return torus;
}
