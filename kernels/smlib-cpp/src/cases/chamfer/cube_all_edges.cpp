// A cube with every edge chamfered. A cube of side 10 between the origin and (10, 10, 10). Chamfer
// all twelve of its edges, each with the same distance 2 on both of its faces. Three chamfers meet
// at each corner, and the corner is cut flat by a triangle: 26 flat faces.
#include "bench.h"

static const double SIDE = 10;
static const double DISTANCE = 2;

CASE("chamfer/cube-all-edges")
{
    SmBrep* cube = nullptr;
    CHECK(SmApiCreateBox, SmVector3d(0, 0, 0), SIDE, SIDE, SIDE, cube);
    // All twelve edges blended at once with the flat section: each a flat cut between the lines
    // where a round of that radius would touch its faces, at a square edge the lines at the radius
    // from it, and each corner closed by its own flat face. (SmApiChamferFillet, the kernel's
    // chamfer of every edge, mitres the corners instead: chamfer/cube-all-edges-mitered.)
    auto edges = bench::edges_of(cube);
    CHECK(SmApiFilletEdges, cube, edges, DISTANCE, bench::LINEAR, bench::G1, 1.0);
    return cube;
}
