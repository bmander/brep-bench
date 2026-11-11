// A block whose top is chamfered into a ridge. A block from the origin to (10, 4, 6). Chamfer the
// two edges of its top face z = 6 that run along x, the one at y = 0 and the one at y = 4, each
// with the same distance 2 on both of its faces, half the top face's width, so that the top face
// vanishes and the chamfers meet along the ridge y = 2, z = 6.
#include "bench.h"

#include <cmath>

static const double LENGTH = 10, WIDTH = 4, HEIGHT = 6;
static const double DISTANCE = 2;
static const double TOLERANCE = 1e-7;

CASE("chamfer/block-ridge")
{
    SmBrep* block = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, HEIGHT, block);
    // The top face's edges along x are the ones through (5, 0, 6) and (5, 4, 6).
    auto edges = bench::edges_where(block, [](SmEdge* edge) {
        SmPoint3d m = bench::middle(edge);
        return std::abs(m.x - LENGTH / 2) < TOLERANCE && std::abs(m.z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(edges.GetSize(), 2, "edges along x at z = 6");
    // A flat cut between the lines where a round of that radius would touch the faces: at a
    // square edge, the lines at the radius from it. The block is blended in place.
    CHECK(SmApiFilletEdges, block, edges, DISTANCE, bench::LINEAR, bench::G1, 1.0);
    return block;
}
