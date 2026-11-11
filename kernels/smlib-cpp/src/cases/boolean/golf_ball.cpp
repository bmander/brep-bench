// A golf ball. A ball of radius 10 centred on the origin, with 62 dimples, each cut by a ball of
// radius 1.2 centred 10.6 out towards one of the 12 vertices, 30 edge midpoints and 20 face centres
// of the icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0) and (±φ, 0, ±1). Each ball's poles on
// the vertical through its centre; each dimple its own cut.
#include "bench.h"

#include <cmath>

static const double RADIUS = 10;
static const double DIMPLE_RADIUS = 1.2;
static const double DIMPLE_CENTRE = 10.6;

/// The icosahedron's vertices, edges' midpoints and faces' centres, as unit vectors.
static std::vector<SmVector3d> directions()
{
    const double phi = (1 + std::sqrt(5.0)) / 2;
    std::vector<SmVector3d> vertices;
    for (double a : {-1.0, 1.0})
        for (double b : {-phi, phi})
            vertices.insert(vertices.end(), {{0, a, b}, {a, b, 0}, {b, 0, a}});
    // neighbours are an edge, 2, apart
    auto edge = [&](size_t i, size_t j)
    { return std::abs((vertices[i] - vertices[j]).Length() - 2) < 1e-9; };
    auto unit = [](SmVector3d v) { return v / v.Length(); };
    std::vector<SmVector3d> out;
    for (const SmVector3d& v : vertices)
        out.push_back(unit(v));
    for (size_t i = 0; i < 12; ++i)
        for (size_t j = i + 1; j < 12; ++j)
            if (edge(i, j))
                out.push_back(unit(vertices[i] + vertices[j]));
    for (size_t i = 0; i < 12; ++i)
        for (size_t j = i + 1; j < 12; ++j)
            for (size_t k = j + 1; k < 12; ++k)
                if (edge(i, j) && edge(j, k) && edge(i, k))
                    out.push_back(unit(vertices[i] + vertices[j] + vertices[k]));
    return out;
}

CASE("boolean/golf-ball")
{
    // SMLib's sphere has its poles above and below its centre, on the vertical through it.
    SmBrep* ball = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), RADIUS, ball);
    std::vector<SmVector3d> dimples = directions();
    bench::expect_count(dimples.size(), 62, "dimples");
    for (const SmVector3d& d : dimples)
    {
        SmBrep* cutter = nullptr;
        CHECK(SmApiCreateSphere, d * DIMPLE_CENTRE, DIMPLE_RADIUS, cutter);
        CHECK(SmApiBooleanDifference, ball, cutter, ball);
    }
    return ball;
}
