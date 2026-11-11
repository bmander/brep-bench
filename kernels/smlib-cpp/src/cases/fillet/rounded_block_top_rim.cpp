// A rounded block with its top rim rounded. A block 20 by 12 by 5, from (−10, −6, 0) to (10, 6, 5),
// with its four vertical edges rounded to radius 3. Round the rim of its top face to radius 1:
// eight edges, four straight and four quarter circles, each running on tangent into the next, the
// round running around all eight unbroken.
#include "bench.h"

#include <cmath>

static const double HALF_LENGTH = 10, HALF_WIDTH = 6, HEIGHT = 5;
static const double CORNER_RADIUS = 3;
static const double RADIUS = 1;
static const double TOLERANCE = 1e-7;

CASE("fillet/rounded-block-top-rim")
{
    SmBrep* block = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-HALF_LENGTH, -HALF_WIDTH, 0), 2 * HALF_LENGTH,
          2 * HALF_WIDTH, HEIGHT, block);
    // The vertical edges are the ones halfway up.
    auto uprights = bench::edges_where(block, [](SmEdge* edge) {
        return std::abs(bench::middle(edge).z - HEIGHT / 2) < TOLERANCE;
    });
    bench::expect_count(uprights.GetSize(), 4, "vertical edges");
    CHECK(SmApiFilletEdges, block, uprights, CORNER_RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    // The top face's rim is every edge lying at z = 5.
    auto rim = bench::edges_where(block, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return std::abs(bench::middle(edge).z - HEIGHT) < TOLERANCE
            && std::abs(ends[0].z - HEIGHT) < TOLERANCE && std::abs(ends[1].z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(rim.GetSize(), 8, "edges around the top face");
    CHECK(SmApiFilletEdges, block, rim, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return block;
}
