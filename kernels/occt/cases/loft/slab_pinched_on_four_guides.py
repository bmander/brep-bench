"""A slab pinched thin on four Bézier guides. A solid lofted through rectangles, its shape set by four guide curves: a slab that is pinched
almost to nothing on its way up, while its long sides turn from x to y.

Every section of the solid at a height z from 0 to 24 is a rectangle in the plane at that height,
centred on the z axis, with its sides parallel to the x and y axes, and its four corners lie on
the four guides. The guide through the corner with x and y positive is the cubic Bézier curve with
control points (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19) and (3, 12.2, 24); the other three are
its mirror images in the xz plane, the yz plane and both (its control points with y, x, or both
negated). Along each guide z rises steadily, so each height meets each guide once.

So the rectangle begins as 24 × 6.4 (x from −12 to 12, y from −3.2 to 3.2) at z = 0. As it rises
it grows longer in x and thinner in y, until at z = 26/3 (the guides' parameter t = 1/3) it is
30 × 0.4, its thinnest; then its length along x shrinks while its width along y grows, and it ends
as 6 × 24.4 at z = 24. The width along y is never less than 0.4.

Each of the four walls is swept by one side of the rectangle, a straight horizontal segment
between two neighbouring guides: the wall facing +x is made of the segments parallel to the y
axis from each point (p, −q, z) of one guide to the point (p, q, z) of the other, and the wall
facing +y of the segments parallel to the x axis from (−p, q, z) to (p, q, z). The ends are flat."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

REACH = 40  # how far each extrusion runs past the solid, either way
# the guide through the corner with x and y positive, (p, q, z): its control points
GUIDE = [(12, 3.2, 0), (18, -2.8, 8), (15, 0.2, 19), (3, 12.2, 24)]


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()


def between(curve, axis, across):
    """Everything within `curve` of the z axis along `axis` (0 for x, 1 for y): the region of the
    plane of `axis` and z between the curve and its mirror image, extruded both ways along the
    other horizontal axis, `across`, far past the solid."""
    mirror = [tuple(-c if i == axis else c for i, c in enumerate(p)) for p in curve]
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(bezier(curve))
    if curve[-1] != mirror[-1]:  # the top: a segment, unless the curves meet there
        wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*curve[-1]), gp_Pnt(*mirror[-1])).Edge())
    wire.Add(bezier(mirror[::-1]))
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*mirror[0]), gp_Pnt(*curve[0])).Edge())
    move = gp_Trsf()
    move.SetTranslation(gp_Vec(*across).Multiplied(-REACH))
    face = BRepBuilderAPI_Transform(BRepBuilderAPI_MakeFace(wire.Wire(), True).Face(), move)
    prism = BRepPrimAPI_MakePrism(face.Shape(), gp_Vec(*across).Multiplied(2 * REACH))
    if not prism.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not extrude the region inside a guide")
    return prism.Shape()


def build():
    # OCCT has no loft along guides (BRepOffsetAPI_MakePipeShell follows one, keeping the
    # section's shape), so the solid is built from what its walls are. The section's sides stay
    # parallel to x and y, so the walls facing ±x are the guide seen along y, (p, z), and its
    # mirror image, each slid along y; those facing ±y are (q, z) and its mirror image, slid along
    # x. The solid is everything between both pairs: the common of the two extrusions.
    x, y = (1, 0, 0), (0, 1, 0)
    seen_along_y = [(p, 0, z) for p, q, z in GUIDE]
    seen_along_x = [(0, q, z) for p, q, z in GUIDE]
    common = BRepAlgoAPI_Common(between(seen_along_y, 0, y), between(seen_along_x, 1, x))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two extrusions")
    return common.Shape()
