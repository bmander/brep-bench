// A disc with six holes indexed around it. A round disc of radius 20 on the z axis, from z = −5 to
// z = 0. Six round holes of radius 2 go straight through it along z, their axes on the circle of
// radius 12 about the z axis: the first at (12, 0), and each of the others the first turned about
// the z axis by 60°, 120°, 180°, 240° and 300°. Cut them one after another, each hole its own cut.
// If you cut a hole with a cylinder, make it run past both faces of the disc.
#include "bench.h"

static const double DISC_RADIUS = 20;
static const double BOTTOM = -5, TOP = 0;
static const double HOLE_RADIUS = 2, PITCH_RADIUS = 12;
static const int COUNT = 6;

CASE("boolean/indexed-pattern")
{
    SmBrep *disc = nullptr, *first = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, BOTTOM), DISC_RADIUS, TOP - BOTTOM, disc);
    // The first drill, at (12, 0), runs 1 past both faces: from z = −6 to z = 1.
    CHECK(SmApiCreateCylinder, SmVector3d(PITCH_RADIUS, 0, BOTTOM - 1), HOLE_RADIUS,
          TOP - BOTTOM + 2, first);
    for (int i = 0; i < COUNT; ++i)
    {
        // A Boolean deletes its second operand and SmApiRotate turns its object in place, so each
        // drill is a copy of the first, turned about the z axis.
        SmBrep* drill = nullptr;
        CHECK(SmApiBrepCopy, first, drill);
        CHECK(SmApiRotate, drill, SmVector3d(0, 0, 0), SmVector3d(0, 0, 1), i * 360.0 / COUNT);
        CHECK(SmApiBooleanDifference, disc, drill, disc);
    }
    return disc;
}
