// What the SMLib C++ implementations share: SM_API's headers, a case's registration, how a
// declined operation becomes a refusal, and a few small readings of a solid's edges.
#pragma once

#include <SmApiBrep.h>
#include <SmApiCurves.h>
#include <SmApiFillets.h>
#include <SmApiGeneral.h>
#include <SmApiHeal.h>
#include <SmApiPrimitives.h>
#include <SmApiQueries.h>
#include <SmApiSurfaces.h>
#include <SmApiTrimmedSurfaces.h>
#include <SmBrep.h>
#include <SmCircle.h>
#include <SmEdge.h>
#include <SmFace.h>
#include <SmLine.h>
#include <SmSurface.h>
#include <SmVertex.h>

#include <stdexcept>
#include <string>
#include <vector>

namespace bench
{
using Build = SmBrep* (*)();

/// SMLib cannot build this case, or one of its operations declined; the message says which.
struct Refused : std::runtime_error
{
    using std::runtime_error::runtime_error;
};

/// SmApiFilletEdges' cross-sections, its lXSectType: flat, the round's two tangent lines joined by
/// a plane, which is a chamfer; and round.
constexpr ULONG LINEAR = 0, CIRCULAR = 1;
/// SmApiFilletEdges' continuity, which only a blend-curve section heeds.
constexpr ULONG G1 = 1;

/// A case's implementation, entered under its id by the CASE macro.
struct Registration
{
    Registration(const char* id, Build build);
};

/// The implementation of a case, or none.
Build find(const std::string& id);

/// SmMessages.h's name for a status.
const char* status_name(SmApiStatus status);

/// An SM_API call that did not succeed declined the operation: the case is refused, naming it.
void check(SmApiStatus status, const char* call);

/// Something the implementation takes as given is not so: a defect, not a refusal.
void expect(bool holds, const std::string& what);

/// There are as many of something as the implementation takes there to be: "3 edges" if not.
void expect_count(size_t count, size_t wanted, const std::string& what);

/// The solid's edges, borrowed from it.
SmTArray<SmEdge*> edges_of(SmBrep* solid);

/// The edges of a solid for which `pick` holds.
template <class Pick> SmTArray<SmEdge*> edges_where(SmBrep* solid, Pick pick)
{
    SmTArray<SmEdge*> picked;
    for (SmEdge* edge : edges_of(solid))
        if (pick(edge))
            picked.Add(edge);
    return picked;
}

/// The faces on either side of an edge.
SmTArray<SmFace*> faces_of(SmEdge* edge);

/// A vertex's point.
SmPoint3d point_of(SmVertex* vertex);

/// The points at an edge's two ends, its start first.
std::vector<SmPoint3d> ends_of(SmEdge* edge);

/// The point halfway along an edge, by its parameter.
SmPoint3d middle(SmEdge* edge);

/// Whether a face's surface is curved: not within SMLib's least distance of a plane.
bool curved(SmFace* face);

/// The open path through the points: a line segment from each to the next, head to tail.
SmTArray<SmCurve*> polyline(const std::vector<SmPoint3d>& points);

/// The closed polygon through the points: a line segment per side, head to tail.
SmTArray<SmCurve*> polygon(const std::vector<SmPoint3d>& corners);
} // namespace bench

/// An SM_API call, refused when it does not succeed: CHECK(SmApiCreateBox, origin, 1, 2, 3, box).
#define CHECK(function, ...) bench::check(function(__VA_ARGS__), #function)

/// The implementation of a case, by its id: CASE("primitives/box") { ...; return solid; }
#define CASE(id)                                                                                   \
    static SmBrep* build();                                                                       \
    static const bench::Registration registration(id, build);                                     \
    static SmBrep* build()
