"""A block whose top is rounded into a half-cylinder. A block from the origin to (10, 4, 6).
Round the two edges of its top face z = 6 that run along x, the one at y = 0 and the one at
y = 4, each to radius 2: half the top face's width of 4.

The two rounds are quarters of the same cylinder of radius 2 on the line y = 2, z = 4, so the top
face vanishes entirely: the rounds meet along the line y = 2, z = 6, tangent to each other there,
and the block's top is a half-cylinder. The two ends x = 0 and x = 10 are each a 4 × 4 square
under a half disc."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

LENGTH, WIDTH, HEIGHT = 10, 4, 6
RADIUS = 2
TOLERANCE = 1e-7


def build():
    block = BRepPrimAPI_MakeBox(LENGTH, WIDTH, HEIGHT).Shape()
    fillet = BRepFilletAPI_MakeFillet(block)
    # The top face's two edges along x: both ends at z = 6, and at the same y.
    edges = TopExp_Explorer(block, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if (abs(a.Z() - HEIGHT) < TOLERANCE and abs(b.Z() - HEIGHT) < TOLERANCE
                and abs(a.Y() - b.Y()) < TOLERANCE):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 2, f"{fillet.NbContours()} top edges along x"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the top's two long edges")
    return fillet.Shape()
