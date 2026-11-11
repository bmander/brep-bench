#include "bench.h"

#include <cmath>
#include <map>

namespace bench
{
static std::map<std::string, Build>& registry()
{
    static std::map<std::string, Build> cases;
    return cases;
}

Registration::Registration(const char* id, Build build) { registry()[id] = build; }

Build find(const std::string& id)
{
    auto it = registry().find(id);
    return it == registry().end() ? nullptr : it->second;
}

const char* status_name(SmApiStatus status)
{
    switch (status)
    {
    case SM_SUCCESS: return "SM_SUCCESS";
    case SM_ERR: return "SM_ERR";
    case SM_ERR_OUT_OF_MEMORY: return "SM_ERR_OUT_OF_MEMORY";
    case SM_ERR_UNKNOWN: return "SM_ERR_UNKNOWN";
    case SM_ERR_FATAL: return "SM_ERR_FATAL";
    case SM_ERR_ASSERT_FAILURE: return "SM_ERR_ASSERT_FAILURE";
    case SM_ERR_NULL_POINTER: return "SM_ERR_NULL_POINTER";
    case SM_ERR_INVALID_INPUT: return "SM_ERR_INVALID_INPUT";
    case SM_ERR_NON_NULL_OUTPUT_POINTER: return "SM_ERR_NON_NULL_OUTPUT_POINTER";
    case SM_ERR_ASSERTVALID_FAILURE: return "SM_ERR_ASSERTVALID_FAILURE";
    case SM_ERR_OUTSIDE_OF_DOMAIN: return "SM_ERR_OUTSIDE_OF_DOMAIN";
    case SM_ERR_NOT_WITHIN_TOLERANCE: return "SM_ERR_NOT_WITHIN_TOLERANCE";
    case SM_ERR_NOT_CONVERGING: return "SM_ERR_NOT_CONVERGING";
    case SM_ERR_WARNING: return "SM_ERR_WARNING";
    case SM_ERR_MESSAGE: return "SM_ERR_MESSAGE";
    case SM_ERR_AXIS_INSIDE_FACE: return "SM_ERR_AXIS_INSIDE_FACE";
    case SM_ERR_BAD_INTERSECTIONS: return "SM_ERR_BAD_INTERSECTIONS";
    case SM_ERR_BAD_COINCIDENT_VERTICES: return "SM_ERR_BAD_COINCIDENT_VERTICES";
    case SM_ERR_BAD_SURFACE_POINT: return "SM_ERR_BAD_SURFACE_POINT";
    case SM_ERR_BAD_TANGENT_DROP: return "SM_ERR_BAD_TANGENT_DROP";
    case SM_ERR_DEGENERATE_SURFACE: return "SM_ERR_DEGENERATE_SURFACE";
    case SM_ERR_BAD_FIND_DEGEN_PARAM: return "SM_ERR_BAD_FIND_DEGEN_PARAM";
    case SM_ERR_METHOD_FAILURE_QUITING: return "SM_ERR_METHOD_FAILURE_QUITING";
    case SM_ERR_NOTYET_HEAL_FACE: return "SM_ERR_NOTYET_HEAL_FACE";
    case SM_ERR_TESS_FAIL_FACE: return "SM_ERR_TESS_FAIL_FACE";
    default: return "an unnamed status";
    }
}

void check(SmApiStatus status, const char* call)
{
    if (status != SM_SUCCESS)
        throw Refused(std::string(call) + " returned " + status_name(status) + " ("
                      + std::to_string(status) + ")");
}

void expect(bool holds, const std::string& what)
{
    if (!holds)
        throw std::logic_error(what);
}

void expect_count(size_t count, size_t wanted, const std::string& what)
{
    expect(count == wanted, std::to_string(count) + " " + what);
}

SmTArray<SmEdge*> edges_of(SmBrep* solid)
{
    SmTArray<SmEdge*> edges;
    CHECK(SmApiGetEdges, solid, edges);
    return edges;
}

SmTArray<SmFace*> faces_of(SmEdge* edge)
{
    SmTArray<SmFace*> faces;
    edge->GetFaces(faces);
    return faces;
}

SmPoint3d point_of(SmVertex* vertex)
{
    SmPoint3d point;
    CHECK(SmApiVertexGetPoint, vertex, point);
    return point;
}

std::vector<SmPoint3d> ends_of(SmEdge* edge)
{
    SmVertex *start = nullptr, *end = nullptr;
    edge->GetVertices(start, end);
    return {point_of(start), point_of(end)};
}

SmPoint3d middle(SmEdge* edge)
{
    double low = 0, high = 0;
    CHECK(SmApiEdgeParameterRange, edge, low, high);
    // SmApiEvaluateCurve writes through the pointers it is given: null asks for nothing.
    SmVector3d point;
    SmVector3d *at = &point, *tangent = nullptr, *curvature = nullptr;
    CHECK(SmApiEvaluateCurve, edge->GetCurve(), (low + high) / 2, at, tangent, curvature);
    return point;
}

bool curved(SmFace* face) { return !face->GetSurface()->IsPlanar(); }

SmTArray<SmCurve*> polyline(const std::vector<SmPoint3d>& points)
{
    SmTArray<SmCurve*> sides;
    for (size_t i = 0; i + 1 < points.size(); ++i)
    {
        SmLine* side = nullptr;
        CHECK(SmApiCreateLineSegment, points[i], points[i + 1], side);
        sides.Add(side);
    }
    return sides;
}

SmTArray<SmCurve*> polygon(const std::vector<SmPoint3d>& corners)
{
    std::vector<SmPoint3d> closed = corners;
    closed.push_back(corners.front());
    return polyline(closed);
}
} // namespace bench
