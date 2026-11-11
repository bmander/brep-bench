// A square boss rounded at its corners and its root, two concave rounds and one convex at each
// corner. A plate, the box from the origin to (40, 30, 5), with a square boss joined to its top:
// the box from (12, 7, 5) to (28, 23, 15). The boss's four upright edges are rounded to radius 3,
// and then its root, where its sides meet the plate's top, now one closed chain, tangent all along,
// to radius 2. The boss's top is left sharp.
#include "bench.h"

#include <cmath>

static const double PLATE[] = {40, 30, 5};
static const SmVector3d BOSS_CORNER(12, 7, 5);
static const double BOSS_SIDE = 16, BOSS_HEIGHT = 10;
static const double UPRIGHT_RADIUS = 3, ROOT_RADIUS = 2;
static const double TOLERANCE = 1e-7;

CASE("fillet/square-boss-root-mixed-corners")
{
    SmBrep *plate = nullptr, *boss = nullptr, *part = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), PLATE[0], PLATE[1], PLATE[2], plate);
    CHECK(SmApiCreateBox, BOSS_CORNER, BOSS_SIDE, BOSS_SIDE, BOSS_HEIGHT, boss);
    CHECK(SmApiBooleanUnion, plate, boss, part);
    const double top = PLATE[2] + BOSS_HEIGHT;

    // The boss's upright edges are the ones halfway up it.
    auto uprights = bench::edges_where(part, [&](SmEdge* edge) {
        return std::abs(bench::middle(edge).z - (PLATE[2] + top) / 2) < TOLERANCE;
    });
    bench::expect_count(uprights.GetSize(), 4, "upright edges");
    CHECK(SmApiFilletEdges, part, uprights, UPRIGHT_RADIUS, bench::CIRCULAR, bench::G1, 1.0);

    // The root is the edges lying in the plate's top, z = 5, that are not the plate's own.
    auto root = bench::edges_where(part, [](SmEdge* edge) {
        SmPoint3d m = bench::middle(edge);
        auto ends = bench::ends_of(edge);
        return std::abs(m.z - PLATE[2]) < TOLERANCE && std::abs(ends[0].z - PLATE[2]) < TOLERANCE
            && std::abs(ends[1].z - PLATE[2]) < TOLERANCE && TOLERANCE < m.x
            && m.x < PLATE[0] - TOLERANCE && TOLERANCE < m.y && m.y < PLATE[1] - TOLERANCE;
    });
    bench::expect_count(root.GetSize(), 8, "edges around the root");
    CHECK(SmApiFilletEdges, part, root, ROOT_RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return part;
}
