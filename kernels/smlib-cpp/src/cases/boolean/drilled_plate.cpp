// A plate drilled nine times. A plate from the origin to (100, 20, 10). Drill nine holes of radius
// 2 through it along z, at y = 10 and x = 10, 20, 30, … 90. Drill them one after another, each hole
// its own cut.
#include "bench.h"

static const double LENGTH = 100, WIDTH = 20, THICKNESS = 10;
static const double HOLE_RADIUS = 2;
static const double HOLE_Y = 10;

CASE("boolean/drilled-plate")
{
    SmBrep* plate = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, THICKNESS, plate);
    for (double x = 10; x < 100; x += 10)  // 10, 20, … 90
    {
        // Each drill runs 1 past both faces of the plate.
        SmBrep* drill = nullptr;
        CHECK(SmApiCreateCylinder, SmVector3d(x, HOLE_Y, -1), HOLE_RADIUS, THICKNESS + 2, drill);
        CHECK(SmApiBooleanDifference, plate, drill, plate);
    }
    return plate;
}
