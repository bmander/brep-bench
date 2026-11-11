// An octahedron with every edge chamfered, four chamfers meeting at each corner. The octahedron |x|
// + |y| + |z| ≤ 10, its six corners on the axes, 10 from the origin. Chamfer all twelve of its
// edges, each with the same distance √6 on both of its faces, measured across the face square to
// the edge: the chamfer of the edge from (10, 0, 0) to (0, 10, 0) is the plane x + y = 8. Four
// chamfers meet at each corner, and the corner is cut flat by a square: 26 flat faces.
#include "bench.h"
#include "octahedron.h"

static const double REACH = 10;  // each corner's distance from the origin
static const double DISTANCE = std::sqrt(6.0);
// The flat section joins the lines where a round would touch the two faces, and the kernel takes
// the round's radius: those lines lie at the radius over the tangent of half the angle between the
// faces from the edge. An octahedron's faces meet at arccos(−1/3).
static const double RADIUS = DISTANCE * std::tan(std::acos(-1.0 / 3) / 2);

CASE("chamfer/octahedron-all-edges")
{
    SmBrep* solid = bench::octahedron(REACH);
    // All twelve edges blended at once with the flat section, each corner closed by its own flat
    // face. (SmApiChamferFillet, the kernel's chamfer of every edge, mitres a cube's corners, and
    // declines the octahedron.)
    auto edges = bench::edges_of(solid);
    CHECK(SmApiFilletEdges, solid, edges, RADIUS, bench::LINEAR, bench::G1, 1.0);
    return solid;
}
