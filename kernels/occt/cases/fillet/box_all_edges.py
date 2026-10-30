"""A cube with every edge rounded. A cube of side 10 with all twelve of its edges rounded to
radius 1. Three rounds meet at each corner."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
RADIUS = 1


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    fillet = BRepFilletAPI_MakeFillet(cube)
    # Every edge of the cube. The explorer meets each edge once from each of its two faces;
    # MakeFillet takes an edge it already holds only once.
    edges = TopExp_Explorer(cube, TopAbs_EDGE)
    while edges.More():
        fillet.Add(RADIUS, TopoDS.Edge(edges.Current()))
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the cube's edges")
    return fillet.Shape()
