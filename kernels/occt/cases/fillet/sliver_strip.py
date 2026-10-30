"""A plate with a sliver of side left between two rounds. A plate from the origin to
(20, 10, 2). Round the two edges of its face y = 10 that run along x, the one at z = 0 and the
one at z = 2, each to radius 0.9999.

What is left of the face y = 10 is a strip 20 long and only 0.0002 tall, from z = 0.9999 to
z = 1.0001, between the two rounds, which are quarters of the cylinders of radius 0.9999 on the
lines y = 9.0001, z = 0.9999 and y = 9.0001, z = 1.0001. The strip is a real face of the solid,
tangent to both rounds."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

LENGTH, WIDTH, THICKNESS = 20, 10, 2
RADIUS = 0.9999
TOLERANCE = 1e-7


def build():
    plate = BRepPrimAPI_MakeBox(LENGTH, WIDTH, THICKNESS).Shape()
    fillet = BRepFilletAPI_MakeFillet(plate)
    # The face y = 10's two edges along x: both ends at y = 10, and at the same z.
    edges = TopExp_Explorer(plate, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if (abs(a.Y() - WIDTH) < TOLERANCE and abs(b.Y() - WIDTH) < TOLERANCE
                and abs(a.Z() - b.Z()) < TOLERANCE):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 2, f"{fillet.NbContours()} edges of y = {WIDTH} along x"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the two edges")
    return fillet.Shape()
