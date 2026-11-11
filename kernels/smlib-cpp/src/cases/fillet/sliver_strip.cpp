// A plate with a sliver of side left between two rounds. A plate from the origin to (20, 10, 2).
// Round the two edges of its face y = 10 that run along x, the one at z = 0 and the one at z = 2,
// each to radius 0.9999, leaving a strip of the face 0.0002 tall between the rounds.
#include "bench.h"

#include <cmath>

static const double LENGTH = 20, WIDTH = 10, THICKNESS = 2;
static const double RADIUS = 0.9999;
static const double TOLERANCE = 1e-7;

CASE("fillet/sliver-strip")
{
    SmBrep* plate = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, THICKNESS, plate);
    // The edges of the face y = 10 along x are the ones through (10, 10, 0) and (10, 10, 2).
    auto edges = bench::edges_where(plate, [](SmEdge* edge) {
        SmPoint3d m = bench::middle(edge);
        return std::abs(m.x - LENGTH / 2) < TOLERANCE && std::abs(m.y - WIDTH) < TOLERANCE;
    });
    bench::expect_count(edges.GetSize(), 2, "edges along x at y = 10");
    CHECK(SmApiFilletEdges, plate, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return plate;
}
