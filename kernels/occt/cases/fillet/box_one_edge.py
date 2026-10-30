"""A cube with one edge rounded. A cube of side 10 between the origin and (10, 10, 10). Round the
edge where its faces x = 10 and y = 10 meet, to radius 2."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
RADIUS = 2
TOLERANCE = 1e-7


def on_both_faces(point):
    """Whether a point lies on the face x = 10 and on the face y = 10."""
    return abs(point.X() - SIDE) < TOLERANCE and abs(point.Y() - SIDE) < TOLERANCE


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    fillet = BRepFilletAPI_MakeFillet(cube)
    # The edge where the faces x = 10 and y = 10 meet is the one whose two ends lie on both.
    edges = TopExp_Explorer(cube, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        first, last = TopExp.FirstVertex_s(edge), TopExp.LastVertex_s(edge)
        if on_both_faces(BRep_Tool.Pnt_s(first)) and on_both_faces(BRep_Tool.Pnt_s(last)):
            fillet.Add(RADIUS, edge)
        edges.Next()
    # The explorer meets the edge once from each of its faces; MakeFillet holds it once.
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} edges at x = y = {SIDE}"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge")
    return fillet.Shape()
