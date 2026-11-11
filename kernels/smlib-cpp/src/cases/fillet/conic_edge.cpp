// A block's edge blended by a parabolic conic. A block, the box from the origin to (20, 10, 10).
// Blend the edge where its top, z = 10, meets its end, x = 20, with a conic blend of rho 0.5, a
// parabola, tangent to the top and to the end each 3 from the edge: in every plane y = c the
// quadratic Bézier curve through (17, c, 10), (20, c, 10) and (20, c, 7).
#include "bench.h"

static const double LENGTH = 20, WIDTH = 10, HEIGHT = 10;  // the block
static const double SETBACK = 3;
static const double MARGIN = 1;

CASE("fillet/conic-edge")
{
    // Workaround: SMLib's fillets have a circular or a flat section, or its own blend curve shaped
    // by a continuity and a thumbweight, and none takes a conic's rho. The same solid is the block
    // less the corner outside the parabola: the parabola, closed around the corner outside the
    // block, swept along y past both ends.
    const double y = -MARGIN;
    const SmPoint3d on_top(LENGTH - SETBACK, y, HEIGHT), corner(LENGTH, y, HEIGHT);
    const SmPoint3d on_end(LENGTH, y, HEIGHT - SETBACK);
    SmTArray<SmPoint3d> controls;
    for (const SmPoint3d& p : {on_top, corner, on_end})
        controls.Add(p);
    SmTArray<double> knots;
    knots.Add(0);
    knots.Add(1);
    SmTArray<ULONG> multiplicities;
    multiplicities.Add(3);
    multiplicities.Add(3);
    SmBSplineCurve* parabola = nullptr;
    CHECK(SmApiCreateCanonicalCurve, controls, knots, multiplicities, nullptr, 2,
          SM_CF_PARABOLIC_ARC, parabola);
    SmTArray<SmCurve*> outline;
    outline.Add(parabola);
    // and around the corner outside the block, from the parabola's end back to its start
    for (SmCurve* side : bench::polyline({on_end, {LENGTH + MARGIN, y, HEIGHT - SETBACK},
                                          {LENGTH + MARGIN, y, HEIGHT + MARGIN},
                                          {LENGTH - SETBACK, y, HEIGHT + MARGIN}, on_top}))
        outline.Add(side);
    SmBrep *outside = nullptr, *block = nullptr;
    CHECK(SmApiCreateLinearSweep, outline, SmVector3d(0, 1, 0), WIDTH + 2 * MARGIN, TRUE, outside);
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), LENGTH, WIDTH, HEIGHT, block);
    CHECK(SmApiBooleanDifference, block, outside, block);
    return block;
}
