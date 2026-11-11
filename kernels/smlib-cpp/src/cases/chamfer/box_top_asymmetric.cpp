// A box with its top edges chamfered unequally. A box from the origin to (20, 10, 6). Chamfer the
// four edges around its top face z = 6, each with distance 1 measured across the top face and
// distance 2 measured down the side face: the chamfer of the edge at y = 10, z = 6 is the plane
// through the lines y = 9, z = 6 and y = 10, z = 4, and the other three likewise.
#include "bench.h"

static const double LENGTH = 20, WIDTH = 10, HEIGHT = 6;
static const double ACROSS_TOP = 1, DOWN_SIDE = 2;
static const double MARGIN = 1;

CASE("chamfer/box-top-asymmetric")
{
    // Workaround: SMLib's chamfers are set back equally on both faces (SmApiChamferFillet takes
    // one width, SmApiFilletEdges' flat section one radius). The same solid is the box less, along
    // each top edge, the wedge beyond its chamfer's plane: a triangle with that plane's line as
    // its long side, reaching past the edge, swept along the edge past both of its ends. (The
    // kernel's cut by a plane leaves the solid open where it cuts.) The wedge is made for the edge
    // at x = 20, along y, and turned about the box's upright axis to each of the others.
    SmBrep* box = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, HEIGHT, box);
    const double turns[] = {0, 90, 180, 270};
    for (int i = 0; i < 4; ++i)
    {
        // the length of the edge, and the box's reach across it
        const double run = i % 2 ? LENGTH : WIDTH, reach = i % 2 ? WIDTH : LENGTH;
        const double x = reach / 2, z = HEIGHT;  // the edge, in a section through the box's middle
        auto outline = bench::polygon({{x - 2 * ACROSS_TOP, 0, z + DOWN_SIDE},
                                       {x + ACROSS_TOP, 0, z + DOWN_SIDE},
                                       {x + ACROSS_TOP, 0, z - 2 * DOWN_SIDE}});
        SmBrep* wedge = nullptr;
        CHECK(SmApiCreateLinearSweep, outline, SmVector3d(0, 1, 0), run + 2 * MARGIN, TRUE, wedge);
        CHECK(SmApiTranslate, wedge, SmVector3d(0, -run / 2 - MARGIN, 0));
        CHECK(SmApiRotate, wedge, SmVector3d(0, 0, 0), SmVector3d(0, 0, 1), turns[i]);
        CHECK(SmApiTranslate, wedge, SmVector3d(LENGTH / 2, WIDTH / 2, 0));
        CHECK(SmApiBooleanDifference, box, wedge, box);
    }
    return box;
}
