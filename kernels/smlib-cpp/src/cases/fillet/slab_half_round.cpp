// A block whose top is rounded into a half-cylinder. A block from the origin to (10, 4, 6). Round
// the two edges of its top face z = 6 that run along x, the one at y = 0 and the one at y = 4, each
// to radius 2, half the top face's width, so that the top face vanishes and the rounds meet along
// the line y = 2, z = 6.
#include "bench.h"

#include <cmath>

static const double LENGTH = 10, WIDTH = 4, HEIGHT = 6;
static const double RADIUS = 2;
static const double TOLERANCE = 1e-7;

CASE("fillet/slab-half-round")
{
    SmBrep* block = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, HEIGHT, block);
    // The top face's edges along x are the ones through (5, 0, 6) and (5, 4, 6).
    auto edges = bench::edges_where(block, [](SmEdge* edge) {
        SmPoint3d m = bench::middle(edge);
        return std::abs(m.x - LENGTH / 2) < TOLERANCE && std::abs(m.z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(edges.GetSize(), 2, "edges along x at z = 6");
    CHECK(SmApiFilletEdges, block, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return block;
}
