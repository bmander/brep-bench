// A cylinder whose rim is chamfered to a point. A cylinder of radius 5 on the z axis, from z = 0 to
// z = 10. Chamfer the circular edge around its top face with the same distance 5 on both faces, the
// cylinder's own radius: the top face shrinks to its centre, and the chamfer is the cone from the
// circle of radius 5 at z = 5 up to its apex at (0, 0, 10).
#include "bench.h"

#include <cmath>

static const double CYLINDER_RADIUS = 5, HEIGHT = 10;
static const double DISTANCE = 5;
static const double TOLERANCE = 1e-7;

CASE("chamfer/cylinder-rim-to-point")
{
    SmBrep* cylinder = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), CYLINDER_RADIUS, HEIGHT, cylinder);
    // The top rim is the edge whose curve is a circle centred at z = 10.
    auto rim = bench::edges_where(cylinder, [](SmEdge* edge) {
        const SmCircle* circle = SM_CAST_PTR(SmCircle, edge->GetCurve());
        SmAxis2Placement frame;
        double radius = 0;
        return circle && circle->GetCanonical(frame, radius) == SM_SUCCESS
            && std::abs(frame.GetOrigin().z - HEIGHT) < TOLERANCE;
    });
    bench::expect_count(rim.GetSize(), 1, "circles at z = 10");
    // A flat cut between the lines where a round of that radius would touch the faces: at a
    // square edge, the lines at the radius from it.
    CHECK(SmApiFilletEdges, cylinder, rim, DISTANCE, bench::LINEAR, bench::G1, 1.0);
    return cylinder;
}
