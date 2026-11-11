// The Bézier dome block with one top edge rounded. The block of `nurbs/bezier-dome`: standing on
// the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides
// upright, and its top the bicubic Bézier patch whose sixteen control points are (20i/3, 20j/3,
// zᵢⱼ) at the heights below. Round the edge where the patch meets the side y = 0, the cubic the
// patch's edge v = 0 draws in that plane, to radius 1: the blend a ball rolling along the edge
// leaves between the freeform top and the flat side, trimmed at its ends by the sides x = 0 and x =
// 20.
#include "bench.h"

#include <cmath>

static const double SIDE = 20;
// z[i][j]: row i at x = 20i/3, column j at y = 20j/3
static const double HEIGHTS[4][4] = {{4, 6, 7, 5},
                                     {6, 14, 12, 7},
                                     {5, 13, 16, 8},
                                     {3, 6, 8, 6}};
static const double RADIUS = 1;
static const double TOLERANCE = 1e-7;

static SmPoint3d net(int i, int j) { return SmPoint3d(SIDE * i / 3, SIDE * j / 3, HEIGHTS[i][j]); }

/// The cubic Bézier curve of four control points.
static SmCurve* bezier(const std::vector<SmPoint3d>& points)
{
    SmTArray<SmPoint3d> controls;
    for (const SmPoint3d& p : points)
        controls.Add(p);
    SmTArray<double> knots;
    knots.Add(0);
    knots.Add(1);
    SmTArray<ULONG> multiplicities;
    multiplicities.Add(4);
    multiplicities.Add(4);
    SmBSplineCurve* curve = nullptr;
    CHECK(SmApiCreateCanonicalCurve, controls, knots, multiplicities, nullptr, 3, SM_CF_UNSPECIFIED,
          curve);
    return curve;
}

static SmBrep* dome_block()
{
    // The patch's four edges, each the Bézier curve of a side of the net, head to tail.
    std::vector<std::vector<SmPoint3d>> rims(4);
    for (int k = 0; k < 4; ++k)
    {
        rims[0].push_back(net(0, k));
        rims[1].push_back(net(k, 3));
        rims[2].push_back(net(3, 3 - k));
        rims[3].push_back(net(3 - k, 0));
    }
    // A surface from a 4 × 4 grid of control points is the bicubic Bézier patch; trimmed by its own
    // four edges it is the top face. The trim takes the surface.
    SmTArray<SmPoint3d> grid;
    for (int i = 0; i < 4; ++i)
        for (int j = 0; j < 4; ++j)
            grid.Add(net(i, j));
    SmSurface* patch = nullptr;
    CHECK(SmApiCreateSurface, grid, 4, 4, patch);
    SmTArray<ULONG> loops;
    loops.Add(4);
    SmTArray<SmCurve*> edges;
    for (const auto& rim : rims)
        edges.Add(bezier(rim));
    SmTArray<SmBrep*> faces;
    SmBrep *top = nullptr, *floor = nullptr;
    CHECK(SmApiTrimSurfaceWith3dCurves, patch, loops, edges, top);
    CHECK(SmApiCreatePlane, SmVector3d(0, 0, 0), SIDE, SIDE, floor);
    faces.Add(top);
    faces.Add(floor);
    // Each side is the flat face between an edge of the patch and the floor.
    for (const auto& rim : rims)
    {
        const SmPoint3d start = rim.front(), end = rim.back();
        const SmPoint3d under_start(start.x, start.y, 0), under_end(end.x, end.y, 0);
        SmTArray<SmCurve*> outline;
        outline.Add(bezier(rim));
        for (SmCurve* side : bench::polyline({end, under_end, under_start, start}))
            outline.Add(side);
        SmBrep* face = nullptr;
        CHECK(SmApiCreatePlanarFaces, outline, face);
        faces.Add(face);
    }
    // Merged, the six faces share their edges and close a shell; healing it classifies what the
    // shell encloses as solid.
    SmBrep* block = nullptr;
    CHECK(SmApiMergeBreps, faces, SM_BO_MERGE, block);
    CHECK(SmApiHealBrep, block);
    SmBoolean solid = FALSE;
    CHECK(SmApiBrepIsManifoldSolid, block, solid);
    bench::expect(solid, "the six faces did not close into a solid");
    return block;
}

CASE("fillet/bezier-dome-one-edge")
{
    SmBrep* block = dome_block();
    // The patch's edge v = 0 is the one curved edge in the plane y = 0.
    auto edges = bench::edges_where(block, [](SmEdge* edge) {
        auto ends = bench::ends_of(edge);
        return !edge->GetCurve()->IsKindOf(SmLine_TYPE) && std::abs(ends[0].y) < TOLERANCE
            && std::abs(ends[1].y) < TOLERANCE;
    });
    bench::expect_count(edges.GetSize(), 1, "curved edges at y = 0");
    CHECK(SmApiFilletEdges, block, edges, RADIUS, bench::CIRCULAR, bench::G1, 1.0);
    return block;
}
