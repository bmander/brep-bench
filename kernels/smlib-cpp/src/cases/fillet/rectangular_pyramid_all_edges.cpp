// A pyramid on a rectangle with every edge rounded, its apex split. A pyramid on the rectangle from
// (−10, −6, 0) to (10, 6, 0), its apex at (0, 0, 12). Round all eight of its edges to radius 3/2.
// The pyramid has no inscribed sphere, so no ball touches all four faces at the apex, and the apex
// splits: a short round between the two long sides, closed by a patch of a sphere at each end.
#include "bench.h"

static const double LENGTH = 20, WIDTH = 12, HEIGHT = 12;
static const double RADIUS = 1.5;

CASE("fillet/rectangular-pyramid-all-edges")
{
    // The kernel's pyramid stands on a square: one on the square of the rectangle's length,
    // narrowed along y to the rectangle's width.
    SmBrep* pyramid = nullptr;
    CHECK(SmApiCreatePyramid, SmVector3d(0, 0, 0), LENGTH, HEIGHT, pyramid);
    CHECK(SmApiScale, pyramid, SmVector3d(1, WIDTH / LENGTH, 1));
    bench::expect_count(bench::edges_of(pyramid).GetSize(), 8, "edges");
    CHECK(SmApiCircularFillet, pyramid, RADIUS);  // rounds every edge, in place
    return pyramid;
}
