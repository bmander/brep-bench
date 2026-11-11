// A notched cube with its inside corner rounded. A cube from the origin to (20, 20, 20) with a
// notch cut from one corner: the cube from (10, 10, 10) to (20, 20, 20) removed. The notch's three
// inside faces meet in three concave edges, which all start at the inside corner (10, 10, 10).
// Round the three to radius 3, all at once: three concave quarter cylinders, and at the corner an
// eighth of a sphere.
#include "bench.h"

#include <cmath>

static const double SIDE = 20, NOTCH = 10;
static const double RADIUS = 3;
static const double TOLERANCE = 1e-7;

static bool at_inside_corner(const SmPoint3d& p)
{
    const double c = SIDE - NOTCH;
    return std::abs(p.x - c) < TOLERANCE && std::abs(p.y - c) < TOLERANCE
        && std::abs(p.z - c) < TOLERANCE;
}

CASE("fillet/notched-cube-inside-corner")
{
    SmBrep *cube = nullptr, *notch = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    const double corner = SIDE - NOTCH;
    CHECK(SmApiCreateBox, SmVector3d(corner, corner, corner), NOTCH, NOTCH, NOTCH, notch);
    CHECK(SmApiBooleanDifference, cube, notch, cube);
    // The three concave edges are the ones that end at the inside corner.
    auto edges = bench::edges_where(cube, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return at_inside_corner(ends[0]) || at_inside_corner(ends[1]);
    });
    bench::expect_count(edges.GetSize(), 3, "edges at the inside corner");
    CHECK(SmApiFilletEdges, cube, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return cube;
}
