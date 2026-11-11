// A cube with one edge rounded. A cube of side 10 between the origin and (10, 10, 10). Round the
// edge where its faces x = 10 and y = 10 meet, to radius 2.
#include "bench.h"

#include <cmath>

static const double SIDE = 10;
static const double RADIUS = 2;
static const double TOLERANCE = 1e-7;

/// Whether a point lies on the face x = 10 and on the face y = 10.
static bool on_both_faces(const SmPoint3d& p)
{
    return std::abs(p.x - SIDE) < TOLERANCE && std::abs(p.y - SIDE) < TOLERANCE;
}

CASE("fillet/box-one-edge")
{
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    // The edge where the faces x = 10 and y = 10 meet is the one whose two ends lie on both.
    auto edges = bench::edges_where(cube, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return on_both_faces(ends[0]) && on_both_faces(ends[1]);
    });
    bench::expect_count(edges.GetSize(), 1, "edges at x = y = 10");
    CHECK(SmApiFilletEdges, cube, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);  // in place
    return cube;
}
