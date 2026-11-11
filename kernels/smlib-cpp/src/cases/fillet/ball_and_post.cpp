// A post standing out of a ball, rounded where it leaves. A ball of radius 10 centred on the
// origin, and a post: a cylinder of radius 4 on the z axis from z = 0 to z = 15, joined to the
// ball. The post leaves the ball on the circle of radius 4 at z = √84. Round that circular edge to
// radius 2, adding material: a torus about the z axis tangent to the ball and to the post.
#include "bench.h"

static const double BALL_RADIUS = 10;
static const double POST_RADIUS = 4, POST_HEIGHT = 15;
static const double RADIUS = 2;

CASE("fillet/ball-and-post")
{
    SmBrep *ball = nullptr, *post = nullptr, *part = nullptr;
    CHECK(SmApiCreateSphere, SmVector3d(0, 0, 0), BALL_RADIUS, ball);
    CHECK(SmApiCreateCylinder, SmVector3d(0, 0, 0), POST_RADIUS, POST_HEIGHT, post);
    CHECK(SmApiBooleanUnion, ball, post, part);
    // Where the post leaves the ball is the edge between two curved faces, the ball and the post's
    // side. (A seam lies in one face; the post's top rim has a flat face beside it.)
    auto junction = bench::edges_where(part, [](SmEdge* edge) {
        auto faces = bench::faces_of(edge);
        return faces.GetSize() == 2 && bench::curved(faces[0]) && bench::curved(faces[1]);
    });
    bench::expect(junction.GetSize() > 0, "no edge between the ball and the post");
    CHECK(SmApiFilletEdges, part, junction, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return part;
}
