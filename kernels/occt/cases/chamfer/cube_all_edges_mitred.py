"""A cube with every edge chamfered, its corners mitred. A cube of side 10 between the origin and (10, 10, 10). Chamfer all twelve of its edges, each with
the same distance 2 on both of its faces: an edge's chamfer is the plane through the two lines,
one on each of its faces, at distance 2 from the edge. No face closes a corner: each chamfer runs
on until it meets the others, so the three at each corner meet in a point, the corner mitred.

The solid is the cube cut by the twelve chamfer planes, x + y ≤ 18 for the edge from (10, 10, 0)
to (10, 10, 10) and the others likewise; `chamfer/cube-all-edges` is this solid cut again by a
plane at each corner. It has 18 flat faces. Each face of the cube becomes the 6 × 6 square inset 2
from its edges, for the top face the square from (2, 2, 10) to (8, 8, 10), as in the cornered
case. Each chamfer is a hexagon, a rectangle 6 long and 2√2 wide with a triangle at each end: the
chamfer of the edge from (10, 10, 0) to (10, 10, 10) has corners (9, 9, 1), (10, 8, 2),
(10, 8, 8), (9, 9, 9), (8, 10, 8) and (8, 10, 2). The three chamfers at the corner (10, 10, 10)
meet at the point (9, 9, 9), each pair along a line from it to a square's corner, such as from
(9, 9, 9) to (8, 8, 10).

It is what chamfering the edges one after another leaves, each edge its own chamfer, ending on
the faces at its ends, the chamfers made before it among them: build it so, with the kernel's
chamfer, in any order."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
DISTANCE = 2
TOLERANCE = 1e-7


def middles():
    """The middle of each of the cube's twelve edges: one coordinate half the side, the other two
    each at 0 or at the side."""
    for axis in range(3):
        for u in (0, SIDE):
            for v in (0, SIDE):
                point = [u, v]
                point.insert(axis, SIDE / 2)
                yield point


def edge_through(solid, point):
    """The edge of `solid` whose middle is `point`. What is left of a cube's edge keeps its middle,
    each chamfer before it cutting as much off either end."""
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        middle = curve.Value((curve.FirstParameter() + curve.LastParameter()) / 2)
        if max(abs(a - b) for a, b in zip((middle.X(), middle.Y(), middle.Z()), point)) < TOLERANCE:
            return edge
        edges.Next()
    raise AssertionError(f"no edge through {point}")


def build():
    # Each edge its own chamfer: a chamfer made alone ends on the faces at its ends, and the
    # chamfers made before it are among them, so the three at a corner mitre. (All twelve in one
    # BRepFilletAPI_MakeChamfer close each corner with a triangle: chamfer/cube-all-edges.)
    solid = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    for point in middles():
        chamfer = BRepFilletAPI_MakeChamfer(solid)
        chamfer.Add(DISTANCE, edge_through(solid, point))
        chamfer.Build()
        if not chamfer.IsDone():
            raise Refused(f"BRepFilletAPI_MakeChamfer could not chamfer the edge through {point}")
        solid = chamfer.Shape()
    return solid
