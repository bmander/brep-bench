// A cube with every edge chamfered, its corners mitred. A cube of side 10 between the origin and
// (10, 10, 10). Chamfer all twelve of its edges, each with the same distance 2 on both of its
// faces. No face closes a corner: each chamfer runs on until it meets the others, so the three at
// each corner meet in a point. It is what chamfering the edges one after another leaves, each edge
// its own chamfer, ending on the faces at its ends, the chamfers made before it among them.
#include "bench.h"

#include <algorithm>
#include <cmath>

static const double SIDE = 10;
static const double DISTANCE = 2;
static const double TOLERANCE = 1e-7;

/// The middle of each of the cube's twelve edges: one coordinate half the side, the other two
/// each at 0 or at the side.
static std::vector<SmPoint3d> middles()
{
    std::vector<SmPoint3d> out;
    for (int axis = 0; axis < 3; ++axis)
        for (double u : {0.0, SIDE})
            for (double v : {0.0, SIDE})
            {
                double rest[] = {u, v};
                double p[3];
                for (int i = 0, k = 0; i < 3; ++i)
                    p[i] = i == axis ? SIDE / 2 : rest[k++];
                out.emplace_back(p[0], p[1], p[2]);
            }
    return out;
}

/// The edge of `solid` whose middle is `point`. What is left of a cube's edge keeps its middle,
/// each chamfer before it cutting as much off either end.
static SmEdge* edge_through(SmBrep* solid, const SmPoint3d& point)
{
    for (SmEdge* edge : bench::edges_of(solid))
    {
        SmVector3d d = bench::middle(edge) - point;
        if (std::max({std::abs(d.x), std::abs(d.y), std::abs(d.z)}) < TOLERANCE)
            return edge;
    }
    throw std::logic_error("no edge through a cube edge's middle");
}

CASE("chamfer/cube-all-edges-mitered")
{
    // Each edge its own chamfer: a flat cut between the lines where a round of that radius would
    // touch the edge's faces, which at a square edge are at the radius from it.
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    for (const SmPoint3d& point : middles())
    {
        SmTArray<SmEdge*> edge;
        edge.Add(edge_through(cube, point));
        CHECK(SmApiFilletEdges, cube, edge, DISTANCE, bench::LINEAR, bench::G1, 1.0);
    }
    return cube;
}
