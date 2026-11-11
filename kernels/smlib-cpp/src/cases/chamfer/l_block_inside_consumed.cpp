// An L-block whose inside corner is chamfered away. An L-shaped block: the polygon through (0, 0,
// 0), (10, 0, 0), (10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10) in the xz plane, extruded along
// +y from y = 0 to y = 10. Chamfer its one concave edge, the line x = 3, z = 3, with the same
// distance 7 on both of its faces, adding material: the chamfer reaches the far edges of both faces
// and consumes them.
#include "bench.h"

#include <cmath>

static const std::vector<SmPoint3d> SECTION = {  // in the xz plane
    {0, 0, 0}, {10, 0, 0}, {10, 0, 3}, {3, 0, 3}, {3, 0, 10}, {0, 0, 10}};
static const double DEPTH = 10;
static const double INSIDE_X = 3, INSIDE_Z = 3;  // the concave edge
static const double DISTANCE = 7;
static const double TOLERANCE = 1e-7;

CASE("chamfer/l-block-inside-consumed")
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
    // A flat face between the lines where a round of that radius would touch the faces: at a
    // square edge, the lines at the radius from it.
    CHECK(SmApiFilletEdges, block, edges, DISTANCE, bench::LINEAR, bench::G1, 1.0);
    return block;
}
