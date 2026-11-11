// A NURBS cylinder with its top rim rounded. A cylinder of radius 5 on the z axis, from z = 0 to z
// = 10, whose side is one rational B-spline surface: the standard rational quadratic circle of nine
// control points, on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1), times a straight line. Its
// ends are flat discs. Round the circular edge where the side meets the top to radius 1.
#include "bench.h"

#include <cmath>

static const double CYLINDER_RADIUS = 5, HEIGHT = 10;
static const double RADIUS = 1;
static const double TOLERANCE = 1e-7;

/// The rational quadratic circle of radius 5 about the z axis at height z: nine control points on
/// the corners and the middles of the sides of the square around it, the corners weighted √2/2.
static SmCurve* circle(double z)
{
    const double square[9][2] = {{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0},
                                 {-1, -1}, {0, -1}, {1, -1}, {1, 0}};
    SmTArray<SmPoint3d> points;
    SmTArray<double> weights;
    for (int i = 0; i < 9; ++i)
    {
        points.Add(SmPoint3d(CYLINDER_RADIUS * square[i][0], CYLINDER_RADIUS * square[i][1], z));
        weights.Add(i % 2 ? std::sqrt(2.0) / 2 : 1.0);
    }
    SmTArray<double> knots;
    SmTArray<ULONG> multiplicities;
    for (double k : {0.0, 0.25, 0.5, 0.75, 1.0})
        knots.Add(k);
    for (ULONG m : {3, 2, 2, 2, 3})
        multiplicities.Add(m);
    SmBSplineCurve* curve = nullptr;
    CHECK(SmApiCreateCanonicalCurve, points, knots, multiplicities, &weights, 2, SM_CF_UNSPECIFIED,
          curve);
    return curve;
}

CASE("fillet/nurbs-cylinder-rim")
{
    // The side is the skin of degree 1 between the circle at z = 0 and the circle at z = 10: the
    // circle's net times a straight line, kept as a B-spline surface with its seam at y = 0, x > 0.
    // The skin's two ends are capped flat. (Swept straight up instead, the circle is recognised and
    // the side made a cylinder.) The skin copies the curves.
    SmTArray<ULONG> curves_per_profile;
    curves_per_profile.Add(1);
    curves_per_profile.Add(1);
    SmTArray<const SmCurve*> profiles;
    profiles.Add(circle(0));
    profiles.Add(circle(HEIGHT));
    SmBrep* cylinder = nullptr;
    CHECK(SmApiCreateSkinPrimitive, curves_per_profile, profiles, 1, TRUE, cylinder, nullptr,
          nullptr, nullptr);
    // The top rim is the edge whose ends are at z = 10; the seam's are at different heights.
    auto rim = bench::edges_where(cylinder, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return std::abs(ends[0].z - HEIGHT) < TOLERANCE && std::abs(ends[1].z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(rim.GetSize(), 1, "edges at z = 10");
    CHECK(SmApiFilletEdges, cylinder, rim, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return cylinder;
}
