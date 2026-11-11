"""The Bézier dome block with one top edge rounded. The block of `nurbs/bezier-dome`: standing on
the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides upright,
and its top the bicubic Bézier patch whose sixteen control points are (20i/3, 20j/3, zᵢⱼ) at the
heights below. Round the edge where the patch meets the side y = 0, the cubic the patch's edge
v = 0 draws in that plane, to radius 1: the blend a ball rolling along the edge leaves between
the freeform top and the flat side, trimmed at its ends by the sides x = 0 and x = 20."""
import usd_brep as sm

SIDE = 20
HEIGHTS = [[4, 6, 7, 5],      # z[i][j]: row i at x = 20i/3, column j at y = 20j/3
           [6, 14, 12, 7],
           [5, 13, 16, 8],
           [3, 6, 8, 6]]
NET = [[(SIDE * i / 3, SIDE * j / 3, HEIGHTS[i][j]) for j in range(4)] for i in range(4)]
RADIUS = 1
TOLERANCE = 1e-7


def bezier(points):
    """The cubic Bézier curve of four control points."""
    return sm.create_canonical_curve(points, [0, 1], [4, 4], degree=3)


def dome_block():
    # The patch's four edges, each the Bézier curve of a side of the net, head to tail.
    rims = [NET[0], [row[3] for row in NET], NET[3][::-1], [row[0] for row in NET][::-1]]
    # A surface from a 4 × 4 grid of control points is the bicubic Bézier patch; trimmed by its
    # own four edges it is the top face.
    patch = sm.create_surface_from_points([point for row in NET for point in row], 4, 4)
    faces = [sm.trim_surface_with_3d_curves(patch, [4], [bezier(rim) for rim in rims]),
             sm.create_plane((0, 0, 0), SIDE, SIDE)]   # the floor
    # Each side is the flat face between an edge of the patch and the floor.
    for rim in rims:
        start, end = rim[0], rim[-1]
        under_start, under_end = (start[0], start[1], 0), (end[0], end[1], 0)
        faces.append(sm.create_planar_faces([bezier(rim),
                                             sm.create_line_segment(end, under_end),
                                             sm.create_line_segment(under_end, under_start),
                                             sm.create_line_segment(under_start, start)]))
    # Merged, the six faces share their edges and close a shell; healing it classifies what the
    # shell encloses as solid.
    block = sm.merge_breps(faces, operation=sm.BooleanOp.MERGE)
    sm.heal_brep(block)
    assert block.is_manifold_solid(), "the six faces did not close into a solid"
    return block


def build():
    block = dome_block()
    # The patch's edge v = 0 is the one curved edge in the plane y = 0.
    edges = [edge for edge in block.edges()
             if not isinstance(edge.curve(), sm.Line)
             and all(abs(vertex.point()[1]) < TOLERANCE for vertex in edge.vertices())]
    assert len(edges) == 1, f"{len(edges)} curved edges at y = 0"
    return sm.fillet_edges(block, edges, radius=RADIUS)
