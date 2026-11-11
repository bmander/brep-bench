// A ring of twelve balls. Twelve balls of radius 3, their centres evenly spaced on the circle of
// radius 10 about the z axis in the plane z = 0, the first at (10, 0, 0). Neighbouring balls
// overlap in a lens; no ball meets any other but its two neighbours. Join them, one after another,
// into one ring.
#include "bench.h"

#include <cmath>

static const int COUNT = 12;
static const double RING = 10, RADIUS = 3;

CASE("boolean/ring-of-balls")
{
    std::vector<SmBrep*> balls;
    for (int i = 0; i < COUNT; ++i)
    {
        const double angle = 2 * M_PI * i / COUNT;
        SmBrep* ball = nullptr;
        CHECK(SmApiCreateSphere, SmVector3d(RING * std::cos(angle), RING * std::sin(angle), 0),
              RADIUS, ball);
        balls.push_back(ball);
    }
    SmBrep* ring = balls[0];
    for (int i = 1; i < COUNT; ++i)
        CHECK(SmApiBooleanUnion, ring, balls[i], ring);
    return ring;
}
