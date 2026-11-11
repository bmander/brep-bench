// A hole whose two rim rounds meet at its waist. A square plate from (−10, −10, 0) to (10, 10, 2),
// with a hole of radius 3 through it on the z axis. Round both circular edges of the hole, on the
// top face and on the bottom face, each to radius 1, half the plate's thickness, so that the hole's
// wall vanishes and the rounds meet on the waist circle at z = 1.
#include "bench.h"

static const double HALF_SIDE = 10, THICKNESS = 2;
static const double HOLE_RADIUS = 3;
static const double RADIUS = 1;

CASE("fillet/hole-waist")
{
    SmBrep *plate = nullptr, *drill = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-HALF_SIDE, -HALF_SIDE, 0), 2 * HALF_SIDE, 2 * HALF_SIDE,
          THICKNESS, plate);
    // The drill runs past both faces of the plate.
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -1), HOLE_RADIUS, THICKNESS + 2, drill);
    CHECK(SmApiBooleanDifference, plate, drill, plate);
    // The hole's two rims are the edges between its wall and a flat face of the plate; the wall's
    // seam has the wall on both sides, and the plate's own edges have none of it.
    auto rims = bench::edges_where(plate, [](SmEdge* edge) {
        auto faces = bench::faces_of(edge);
        return faces.GetSize() == 2 && bench::curved(faces[0]) != bench::curved(faces[1]);
    });
    bench::expect_count(rims.GetSize(), 2, "rims");
    CHECK(SmApiFilletEdges, plate, rims, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return plate;
}
