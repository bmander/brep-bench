// A box: a rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and
// the opposite corner at (10, 20, 30).
#include "bench.h"

static const SmVector3d CORNER(0, 0, 0);
static const double LENGTH = 10, WIDTH = 20, HEIGHT = 30;

CASE("primitives/box")
{
    SmBrep* box = nullptr;
    CHECK(SmApiCreateBox, CORNER, LENGTH, WIDTH, HEIGHT, box);
    return box;
}
