// An L-block whose inside corner is rounded away. An L-shaped block: the polygon through (0, 0, 0),
// (10, 0, 0), (10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10) in the xz plane, extruded along +y
// from y = 0 to y = 10. Round its one concave edge, the line x = 3, z = 3, to radius 7, adding
// material: the round reaches the far edges of both of its faces and consumes them.
#include "bench.h"

#include <cmath>

static const std::vector<SmPoint3d> SECTION = {  // in the xz plane
    {0, 0, 0}, {10, 0, 0}, {10, 0, 3}, {3, 0, 3}, {3, 0, 10}, {0, 0, 10}};
static const double DEPTH = 10;
static const double INSIDE_X = 3, INSIDE_Z = 3;  // the concave edge
static const double RADIUS = 7;
static const double TOLERANCE = 1e-7;

CASE("fillet/l-block-inside-consumed")
{
    SmBrep* block = nullptr;
    CHECK(SmApiCreateLinearSweep, bench::polygon(SECTION), SmVector3d(0, 1, 0), DEPTH, TRUE, block);
    auto edges = bench::edges_where(block, [](SmEdge* edge) {
        for (const SmPoint3d& p : bench::ends_of(edge))
            if (std::abs(p.x - INSIDE_X) > TOLERANCE || std::abs(p.z - INSIDE_Z) > TOLERANCE)
                return false;
        return true;
    });
    bench::expect_count(edges.GetSize(), 1, "edges on the inside corner");
    CHECK(SmApiFilletEdges, block, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return block;
}
