// A cylinder whose rim is rounded into a dome. A cylinder of radius 5 on the z axis, from z = 0 to
// z = 10. Round the circular edge around its top face to radius 5, the cylinder's own radius, so
// that the top face shrinks to nothing and the round is a hemisphere.
#include "bench.h"

#include <cmath>

static const double CYLINDER_RADIUS = 5, HEIGHT = 10;
static const double RADIUS = 5;
static const double TOLERANCE = 1e-7;

CASE("fillet/cylinder-rim-to-dome")
{
    SmBrep* cylinder = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), CYLINDER_RADIUS, HEIGHT, cylinder);
    // The cylinder has three edges: its two rims and the seam of its side. The top rim is the
    // edge whose curve is a circle centred at z = 10.
    auto rim = bench::edges_where(cylinder, [](SmEdge* edge) {
        const SmCircle* circle = SM_CAST_PTR(SmCircle, edge->GetCurve());
        SmAxis2Placement frame;
        double radius = 0;
        return circle && circle->GetCanonical(frame, radius) == SM_SUCCESS
            && std::abs(frame.GetOrigin().z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(rim.GetSize(), 1, "circles at z = 10");
    CHECK(SmApiFilletEdges, cylinder, rim, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return cylinder;
}
