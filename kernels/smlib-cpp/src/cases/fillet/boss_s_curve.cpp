// A boss whose root and rim rounds meet. A square plate from (−20, −20, 0) to (20, 20, 4), and a
// round boss on it: a cylinder of radius 5 on the z axis from z = 4 to z = 8, joined to the plate.
// Round both circular edges of the boss to radius 2: the rim of its top face (convex) and its root,
// where it meets the plate (concave). Each round takes 2 of the boss's height of 4, so its side
// vanishes and the rounds meet on the circle of radius 5 at z = 6: the boss's profile is an S.
#include "bench.h"

static const double HALF_SIDE = 20, THICKNESS = 4;
static const double BOSS_RADIUS = 5, BOSS_HEIGHT = 4;
static const double RADIUS = 2;

CASE("fillet/boss-s-curve")
{
    SmBrep *plate = nullptr, *boss = nullptr, *part = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(-HALF_SIDE, -HALF_SIDE, 0), 2 * HALF_SIDE, 2 * HALF_SIDE,
          THICKNESS, plate);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, THICKNESS), BOSS_RADIUS, BOSS_HEIGHT, boss);
    CHECK(SmApiBooleanUnion, plate, boss, part);
    // The boss's root and rim are the edges between its side and a flat face, the plate's top or
    // its own; the side's seam has the side on both sides, and the plate's edges have none of it.
    auto circles = bench::edges_where(part, [](SmEdge* edge) {
        auto faces = bench::faces_of(edge);
        return faces.GetSize() == 2 && bench::curved(faces[0]) != bench::curved(faces[1]);
    });
    bench::expect_count(circles.GetSize(), 2, "circles around the boss");
    CHECK(SmApiFilletEdges, part, circles, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return part;
}
