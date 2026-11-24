"""A plate with every edge rounded to half its thickness. A plate from the origin to (40, 30, 6)
with all twelve of its edges rounded to radius 3, half its thickness, in one operation.

The side faces vanish: the top and bottom rounds along each side are the two halves of one
half-cylinder, and at each corner the balls of the top and bottom vertices are one, so the upright
edges keep no round."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

LENGTH, WIDTH, THICKNESS = 40, 30, 6
RADIUS = THICKNESS / 2


def build():
    plate = BRepPrimAPI_MakeBox(LENGTH, WIDTH, THICKNESS).Shape()
    fillet = BRepFilletAPI_MakeFillet(plate)
    # Every edge of the plate. The explorer meets each edge once from each of its two faces;
    # MakeFillet takes an edge it already holds only once.
    edges = TopExp_Explorer(plate, TopAbs_EDGE)
    while edges.More():
        fillet.Add(RADIUS, TopoDS.Edge(edges.Current()))
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the plate's edges")
    return fillet.Shape()
