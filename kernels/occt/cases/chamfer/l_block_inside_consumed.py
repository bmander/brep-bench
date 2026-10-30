"""An L-block whose inside corner is chamfered away. An L-shaped block. Its section in the xz
plane is the polygon through (0, 0, 0), (10, 0, 0), (10, 0, 3), (3, 0, 3), (3, 0, 10) and
(0, 0, 10); extrude it along +y from y = 0 to y = 10. The block has one concave edge: the inside
corner, the line x = 3, z = 3 along y, where the face z = 3 (from x = 3 to x = 10) meets the face
x = 3 (from z = 3 to z = 10).

Chamfer that inside edge with the same distance 7 on both of its faces, adding material: the
chamfer is the plane through the lines (10, y, 3) and (3, y, 10), the far edges of both faces, so
it consumes both faces entirely. The section becomes the pentagon through (0, 0), (10, 0),
(10, 3), (3, 10) and (0, 10), in xz."""
from OCP.BRep import BRep_Tool
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

SECTION = [(0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10)]  # (x, z)
DEPTH = 10
INSIDE = (3, 3)  # the inside corner's x and z
DISTANCE = 7
TOLERANCE = 1e-7


def at_inside_corner(point):
    return abs(point.X() - INSIDE[0]) < TOLERANCE and abs(point.Z() - INSIDE[1]) < TOLERANCE


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in SECTION:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    block = BRepPrimAPI_MakePrism(section, gp_Vec(0, DEPTH, 0)).Shape()

    chamfer = BRepFilletAPI_MakeChamfer(block)
    # The inside edge runs along y; both its ends are at x = 3, z = 3.
    edges = TopExp_Explorer(block, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if (at_inside_corner(BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge)))
                and at_inside_corner(BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge)))):
            chamfer.Add(DISTANCE, edge)
        edges.Next()
    assert chamfer.NbContours() == 1, f"{chamfer.NbContours()} edges at the inside corner"
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the inside edge")
    return chamfer.Shape()
