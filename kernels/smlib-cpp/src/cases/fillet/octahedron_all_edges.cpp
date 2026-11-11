// An octahedron with every edge rounded, four rounds meeting at each corner. The octahedron |x| +
// |y| + |z| ≤ 10, its six corners on the axes, 10 from the origin. Round all twelve of its edges to
// radius 2: at each corner the ball that touches all four faces there closes the corner with a
// patch of its sphere.
#include "bench.h"
#include "octahedron.h"

static const double REACH = 10;  // each corner's distance from the origin
static const double RADIUS = 2;

CASE("fillet/octahedron-all-edges")
{
    SmBrep* octahedron = bench::octahedron(REACH);
    CHECK(SmApiCircularFillet, octahedron, RADIUS);  // rounds every edge, in place
    return octahedron;
}
