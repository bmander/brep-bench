// An L-block's end rounded across its inside corner, one concave round and two convex. The union of
// the box from the origin to (20, 10, 6), its foot, and the box from the origin to (6, 10, 20), its
// upright. The concave edge from (6, 0, 6) to (6, 10, 6) meets, at its end y = 0, two convex edges
// of the end face y = 0: up the upright to (6, 0, 20) and along the foot to (20, 0, 6). All three
// are rounded to radius 2, one after another: the concave edge first, and then the chain it leaves
// on the end face, up the upright, around the concave round's end and along the foot.
#include "bench.h"

#include <cmath>

static const double FOOT[] = {20, 10, 6};
static const double UPRIGHT[] = {6, 10, 20};
static const double RADIUS = 2;
static const double TOLERANCE = 1e-7;

CASE("fillet/l-block-end-mixed-corner")
{
    SmBrep *foot = nullptr, *upright = nullptr, *block = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), FOOT[0], FOOT[1], FOOT[2], foot);
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), UPRIGHT[0], UPRIGHT[1], UPRIGHT[2], upright);
    CHECK(SmApiBooleanUnion, foot, upright, block);
    // The concave edge, where the upright's face x = 6 meets the foot's top z = 6.
    auto concave = bench::edges_where(block, [](SmEdge* edge) {
        for (const SmPoint3d& p : bench::ends_of(edge))
            if (std::abs(p.x - UPRIGHT[0]) > TOLERANCE || std::abs(p.z - FOOT[2]) > TOLERANCE)
                return false;
        return true;
    });
    bench::expect_count(concave.GetSize(), 1, "concave edges");
    CHECK(SmApiFilletEdges, block, concave, RADIUS, bench::CIRCULAR, bench::G1, 1.0);

    // The chain the round leaves on the end face y = 0: the edges lying in that face that are not
    // on the block's outside, x = 0 or 20 and z = 0 or 20.
    auto chain = bench::edges_where(block, [](SmEdge* edge) {
        SmPoint3d m = bench::middle(edge);
        auto ends = bench::ends_of(edge);
        return std::abs(m.y) < TOLERANCE && std::abs(ends[0].y) < TOLERANCE
            && std::abs(ends[1].y) < TOLERANCE && TOLERANCE < m.x && m.x < FOOT[0] - TOLERANCE
            && TOLERANCE < m.z && m.z < UPRIGHT[2] - TOLERANCE;
    });
    bench::expect_count(chain.GetSize(), 3, "edges in the end face's chain");
    CHECK(SmApiFilletEdges, block, chain, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return block;
}
