"""A square spire on four Bézier ridges. A square spire, swelling and then closing to a point: a solid lofted from a square at its foot to
a single point at its top, its shape set by four guide curves, its ridges.

Every section of the solid at a height z from 0 to 24 is a square in the plane at that height,
centred on the z axis, with its sides parallel to the x and y axes, and its four corners lie on
the four ridges. The ridge through the corner with x and y positive is the cubic Bézier curve with
control points (9, 9, 0), (15, 15, 8), (12, 12, 19) and (0, 0, 24), in the vertical plane x = y;
the other three are its mirror images in the xz plane, the yz plane and both (its control points
with y, x, or both negated). Along each ridge z rises steadily, so each height meets each ridge
once.

So the square begins as 18 × 18 at z = 0, swells to 24 × 24 at z = 26/3 (the ridges' parameter
t = 1/3), and then narrows to nothing: the four ridges meet at the apex (0, 0, 24), arriving there
at a slant, so the spire comes to a sharp point.

Each of the four walls is swept by one side of the square, a straight horizontal segment between
two neighbouring ridges: the wall facing +x is made of the segments parallel to the y axis from
each point (g, −g, z) of one ridge to the point (g, g, z) of the other. The four walls meet only
along the ridges and at the apex. The floor is flat."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

REACH = 40  # how far each extrusion runs past the solid, either way
# the ridge through the corner with x and y positive, (g, g, z): its control points
GUIDE = [(9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24)]


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
