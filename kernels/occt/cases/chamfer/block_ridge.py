"""A block whose top is chamfered into a ridge. A block from the origin to (10, 4, 6). Chamfer
the two edges of its top face z = 6 that run along x, the one at y = 0 and the one at y = 4, each
with the same distance 2 on both of its faces: half the top face's width of 4.

The chamfer of the edge at y = 0 is the plane through the lines y = 2, z = 6 and y = 0, z = 4,
and the other's the plane through y = 2, z = 6 and y = 4, z = 4, so the top face vanishes
entirely: the two chamfers meet at right angles along the ridge y = 2, z = 6, and the block's
ends x = 0 and x = 10 are pentagons, a 4 × 4 square under a triangle."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

LENGTH, WIDTH, HEIGHT = 10, 4, 6
DISTANCE = 2
TOLERANCE = 1e-7


def build():
    block = BRepPrimAPI_MakeBox(LENGTH, WIDTH, HEIGHT).Shape()
    chamfer = BRepFilletAPI_MakeChamfer(block)
    # The top face's two edges along x: both ends at z = 6, and at the same y.
    edges = TopExp_Explorer(block, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if (abs(a.Z() - HEIGHT) < TOLERANCE and abs(b.Z() - HEIGHT) < TOLERANCE
                and abs(a.Y() - b.Y()) < TOLERANCE):
            chamfer.Add(DISTANCE, edge)
        edges.Next()
    assert chamfer.NbContours() == 2, f"{chamfer.NbContours()} top edges along x"
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the top's two long edges")
    return chamfer.Shape()
