// A cube edge rounded with a radius growing along it. A cube of side 10 between the origin and (10,
// 10, 10). Round the edge where its faces x = 10 and y = 10 meet with a radius that grows linearly
// along it, r = 1 + z/10: 1 at the bottom face z = 0 and 2 at the top face z = 10.
#include "bench.h"

#include <cmath>

static const double SIDE = 10;
static const double RADIUS_BOTTOM = 1, RADIUS_TOP = 2;
static const double TOLERANCE = 1e-7;

/// Whether a point lies on the face x = 10 and on the face y = 10.
static bool on_both_faces(const SmPoint3d& p)
{
    return std::abs(p.x - SIDE) < TOLERANCE && std::abs(p.y - SIDE) < TOLERANCE;
}

CASE("fillet/variable-radius-edge")
{
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    auto edges = bench::edges_where(cube, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return on_both_faces(ends[0]) && on_both_faces(ends[1]);
    });
    bench::expect_count(edges.GetSize(), 1, "edges at x = y = 10");
    // The radius runs linearly from the edge's start to its end: give each end its own.
    auto ends = bench::ends_of(edges[0]);
    const bool upward = ends[0].z < ends[1].z;
    CHECK(SmApiVariableRadiusFillet, cube, edges, upward ? RADIUS_BOTTOM : RADIUS_TOP,
          upward ? RADIUS_TOP : RADIUS_BOTTOM, bench::CIRCULAR, bench::G1);
    return cube;
}
