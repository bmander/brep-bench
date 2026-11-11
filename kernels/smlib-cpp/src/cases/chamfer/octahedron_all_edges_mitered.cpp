// An octahedron with every edge chamfered, its corners mitred. The octahedron |x| + |y| + |z| ≤ 10,
// its six corners on the axes, 10 from the origin. Chamfer all twelve of its edges, each with the
// same distance √6 on both of its faces, measured across the face square to the edge: the chamfer
// of the edge from (10, 0, 0) to (0, 10, 0) is the plane x + y = 8. No face closes a corner: the
// four chamfers at each corner meet in a point. It is what chamfering the edges one after another
// leaves, each edge its own chamfer, ending on the faces at its ends, the chamfers made before it
// among them.
#include "bench.h"
#include "octahedron.h"

static const double REACH = 10;  // each corner's distance from the origin
static const double DISTANCE = std::sqrt(6.0);
// The flat section joins the lines where a round would touch the two faces, and the kernel takes
// the round's radius: those lines lie at the radius over the tangent of half the angle between the
// faces from the edge. An octahedron's faces meet at arccos(−1/3).
static const double RADIUS = DISTANCE * std::tan(std::acos(-1.0 / 3) / 2);
static const double TOLERANCE = 1e-7;

/// The edge of `solid` lying on the segment from `a` to `b`: what is left of the octahedron's edge
/// there, the chamfers before it having cut its ends off.
static SmEdge* edge_between(SmBrep* solid, const SmPoint3d& a, const SmPoint3d& b)
{
    auto on_segment = [&](const SmPoint3d& p)
    { return std::abs((p - a).Length() + (b - p).Length() - (b - a).Length()) < TOLERANCE; };
    for (SmEdge* edge : bench::edges_of(solid))
    {
        auto ends = bench::ends_of(edge);
        if (on_segment(ends[0]) && on_segment(ends[1]))
            return edge;
    }
    throw std::logic_error("no edge between two of the octahedron's corners");
}

CASE("chamfer/octahedron-all-edges-mitered")
{
    SmBrep* solid = bench::octahedron(REACH);
    std::vector<SmPoint3d> corners;
    for (int axis = 0; axis < 3; ++axis)
        for (double sign : {1.0, -1.0})
        {
            SmPoint3d corner(0, 0, 0);
            corner[axis] = sign * REACH;
            corners.push_back(corner);
        }
    // The ends of each of the octahedron's twelve edges: two corners on different axes, not
    // opposite. Each edge its own chamfer.
    for (size_t i = 0; i < corners.size(); ++i)
        for (size_t j = i + 1; j < corners.size(); ++j)
            if ((corners[i] - corners[j]).Length() < 2 * REACH - TOLERANCE)
            {
                SmTArray<SmEdge*> edge;
                edge.Add(edge_between(solid, corners[i], corners[j]));
                CHECK(SmApiFilletEdges, solid, edge, RADIUS, bench::LINEAR, bench::G1, 1.0);
            }
    return solid;
}
