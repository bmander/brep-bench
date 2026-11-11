// A tee of two cylinders with its junction rounded. The cross bar, a cylinder of radius 6 about the
// x axis from x = −20 to x = 20, joined with the stem, a cylinder of radius 3 about the z axis from
// z = 0 to z = 15. Their sides meet in a closed saddle-shaped curve, the junction. Round it to
// radius 1: the concave blend a ball of radius 1 sweeps rolling around the junction.
#include "bench.h"

static const double BAR_RADIUS = 6, BAR_HALF_LENGTH = 20;
static const double STEM_RADIUS = 3, STEM_HEIGHT = 15;
static const double RADIUS = 1;

CASE("fillet/tee-of-two-cylinders")
{
    // A cylinder stands on the z axis: the bar is made there and laid along x.
    SmBrep *bar = nullptr, *stem = nullptr, *tee = nullptr;
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, -BAR_HALF_LENGTH), BAR_RADIUS, 2 * BAR_HALF_LENGTH,
          bar);
    CHECK(SmApiRotate, bar, SmVector3d(0, 0, 0), SmVector3d(0, 1, 0), 90);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), STEM_RADIUS, STEM_HEIGHT, stem);
    CHECK(SmApiBooleanUnion, bar, stem, tee);
    // The junction is the edges between two curved faces, the bar's side and the stem's. (A seam
    // lies in one face; the rims each have a flat end beside them.)
    auto junction = bench::edges_where(tee, [](SmEdge* edge) {
        auto faces = bench::faces_of(edge);
        return faces.GetSize() == 2 && bench::curved(faces[0]) && bench::curved(faces[1]);
    });
    bench::expect(junction.GetSize() > 0, "no edge between the bar and the stem");
    CHECK(SmApiFilletEdges, tee, junction, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return tee;
}
