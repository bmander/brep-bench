"""A cube with every edge chamfered. A cube of side 10 between the origin and (10, 10, 10).
Chamfer all twelve of its edges, each with the same distance 2 on both of its faces: an edge's
chamfer is the plane through the two lines, one on each of its faces, at distance 2 from the
edge. Three chamfers meet at each corner, and the corner is cut flat by a triangle.

The result has 26 flat faces. Each face of the cube becomes the 6 × 6 square inset 2 from its
edges, for the top face the square from (2, 2, 10) to (8, 8, 10). Each chamfer is a rectangle 6
long and 2√2 wide, between two of those squares: the chamfer of the edge from (10, 10, 0) to
(10, 10, 10) is the rectangle with corners (10, 8, 2), (8, 10, 2), (8, 10, 8) and (10, 8, 8).
Each corner is the equilateral triangle through the three square corners nearest it: at the
corner (10, 10, 10) the triangle through (8, 8, 10), (8, 10, 8) and (10, 8, 8), in the plane
x + y + z = 26."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
DISTANCE = 2


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    chamfer = BRepFilletAPI_MakeChamfer(cube)
    # Every edge of the cube; MakeChamfer takes an edge it already holds only once.
    edges = TopExp_Explorer(cube, TopAbs_EDGE)
    while edges.More():
        chamfer.Add(DISTANCE, TopoDS.Edge(edges.Current()))
        edges.Next()
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the cube's edges")
    return chamfer.Shape()
