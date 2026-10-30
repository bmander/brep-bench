"""An L-block whose inside corner is rounded away. An L-shaped block. Its section in the xz plane
is the polygon through (0, 0, 0), (10, 0, 0), (10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10);
extrude it along +y from y = 0 to y = 10. The block has one concave edge: the inside corner, the
line x = 3, z = 3 along y, where the face z = 3 (from x = 3 to x = 10) meets the face x = 3 (from
z = 3 to z = 10).

Round that inside edge to radius 7, adding material. The ball rolling in the corner touches the
face z = 3 at x = 10 and the face x = 3 at z = 10, the far edges of both faces, so the round
consumes both faces entirely: the round is a quarter of the cylinder of radius 7 on the line
x = 10, z = 10, and it meets the faces x = 10 and z = 10 at right angles along the lines
(10, y, 3) and (3, y, 10). The section becomes the square from (0, 0) to (10, 10) less the
quarter disc of radius 7 centred at its corner (10, 10)."""
from OCP.BRep import BRep_Tool
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

SECTION = [(0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10)]  # (x, z)
DEPTH = 10
INSIDE = (3, 3)  # the inside corner's x and z
RADIUS = 7
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

    fillet = BRepFilletAPI_MakeFillet(block)
    # The inside edge runs along y; both its ends are at x = 3, z = 3.
    edges = TopExp_Explorer(block, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if (at_inside_corner(BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge)))
                and at_inside_corner(BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge)))):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} edges at the inside corner"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the inside edge")
    return fillet.Shape()
