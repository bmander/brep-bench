"""A square-to-round transition three times larger. The transition piece of loft/square-to-round
with every length times 3: from the square −60 ≤ x ≤ 60, −60 ≤ y ≤ 60 in the plane z = 0 to the
circle of radius 45 in the plane z = 90, centred on the z axis. Each side of the square is joined
by a flat triangle to the point of the circle nearest it, and each corner to the quarter circle
between the two points beside it by a piece of an oblique cone, its apex the corner: the convex
hull of the square and the circle."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeSolid,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.TopAbs import TopAbs_FACE, TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

HALF = 60  # half the square's side
RADIUS = 45
HEIGHT = 90


def flat(corners):
    """The flat face bounded by the polygon through `corners`."""
    poly = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        poly.Add(gp_Pnt(*corner))
    poly.Close()
    return BRepBuilderAPI_MakeFace(poly.Wire(), True).Face()


def cone(apex, start):
    """The piece of the cone from the point `apex` to the quarter of the circle that starts at
    the angle `start` (radians from +x) and turns a quarter turn counter-clockwise: a ruled loft
    from a point to the arc, each point of the arc joined to the apex by a straight line."""
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, 0, HEIGHT), gp_Dir(0, 0, 1)), RADIUS)
    arc = BRepBuilderAPI_MakeEdge(circle, start, start + math.pi / 2).Edge()
    loft = BRepOffsetAPI_ThruSections(False, True)  # a shell, ruled
    loft.AddVertex(BRepBuilderAPI_MakePolygon(gp_Pnt(*apex), gp_Pnt(*apex)).FirstVertex())
    loft.AddWire(BRepBuilderAPI_MakeWire(arc).Wire())
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft a corner to its quarter circle")
    return TopoDS.Face(TopExp_Explorer(loft.Shape(), TopAbs_FACE).Current())


def build():
    # OCCT's loft joins sections edge to edge, so one loft of the square to the circle cannot
    # pair each side with a point and each corner with an arc: each cone is lofted from its
    # corner, each triangle is flat, and the hull is sewn from them, the floor and the disc.
    h = HALF
    corners = [(h, -h, 0), (h, h, 0), (-h, h, 0), (-h, -h, 0)]  # counter-clockwise
    faces = [flat(list(reversed(corners)))]
    top = gp_Circ(gp_Ax2(gp_Pnt(0, 0, HEIGHT), gp_Dir(0, 0, 1)), RADIUS)
    faces.append(BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(top).Edge()).Wire(), True).Face())
    for k, corner in enumerate(corners):
        # the corner at angle −45° + 90°k sees the quarter circle from −90° + 90°k
        start = -math.pi / 2 + k * math.pi / 2
        faces.append(cone(corner, start))
        # the triangle from the side before this corner to the circle's point nearest that side
        near = (RADIUS * math.cos(start), RADIUS * math.sin(start), HEIGHT)
        faces.append(flat([corners[k - 1], corner, near]))

    sewing = BRepBuilderAPI_Sewing(1e-6)
    for face in faces:
        sewing.Add(face)
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the faces")
    make = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not make.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid made no solid of the sewn shell")
    solid = make.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid
