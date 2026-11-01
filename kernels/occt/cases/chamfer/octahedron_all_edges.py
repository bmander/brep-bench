"""An octahedron with every edge chamfered, four chamfers meeting at each corner. The octahedron |x| + |y| + |z| ≤ 10: its six corners on the axes, 10 from the origin, such as
(10, 0, 0), and its eight faces equilateral triangles, such as the one in the plane
x + y + z = 10. Chamfer all twelve of its edges, each with the same distance √6 on both of its
faces, measured across the face square to the edge: the chamfer of the edge from (10, 0, 0) to
(0, 10, 0) is the plane x + y = 8, through the two lines, one on each of its faces, at distance √6
from the edge (on the face x + y + z = 10, the line z = 2). Four chamfers meet at each corner,
and the corner is cut flat by a square.

The solid is the octahedron cut by the twelve chamfer planes, |x| + |y| ≤ 8, |y| + |z| ≤ 8 and
|z| + |x| ≤ 8, and by the six corner planes |x| ≤ 6, |y| ≤ 6 and |z| ≤ 6. It has 26 flat faces.
Each face of the octahedron becomes the equilateral triangle of side 4√2 inset from its edges, for
the face x + y + z = 10 the triangle through (6, 2, 2), (2, 6, 2) and (2, 2, 6). Each chamfer is a
rectangle 4√2 long and 4 wide, between two of those triangles: the chamfer x + y = 8 has corners
(6, 2, 2), (2, 6, 2), (2, 6, −2) and (6, 2, −2). Each corner is the square through the four
triangle corners nearest it: at the corner (10, 0, 0) the square of side 4 in the plane x = 6,
through (6, 2, 2), (6, −2, 2), (6, −2, −2) and (6, 2, −2).

Those four corners lie in one plane by the octahedron's symmetry. Where they do not, as at the
apex of a pyramid on a rectangle, no flat face closes the corner, and only the mitre
(`chamfer/octahedron-all-edges-mitred`) is one solid."""
from itertools import product
from math import sqrt

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused

SIZE = 10
DISTANCE = sqrt(6)


def octahedron():
    """The octahedron |x| + |y| + |z| <= 10, sewn from its eight triangles."""
    sewing = BRepBuilderAPI_Sewing()
    for sx, sy, sz in product((1, -1), repeat=3):
        corners = (gp_Pnt(sx * SIZE, 0, 0), gp_Pnt(0, sy * SIZE, 0), gp_Pnt(0, 0, sz * SIZE))
        polygon = BRepBuilderAPI_MakePolygon(*corners, True)
        sewing.Add(BRepBuilderAPI_MakeFace(polygon.Wire()).Face())
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(sewing.SewedShape()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid could not close the octahedron")
    return solid.Shape()


def build():
    solid = octahedron()
    chamfer = BRepFilletAPI_MakeChamfer(solid)
    # Every edge of the octahedron; MakeChamfer takes an edge it already holds only once.
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        chamfer.Add(DISTANCE, TopoDS.Edge(edges.Current()))
        edges.Next()
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the octahedron's edges, four "
                      "meeting at each corner")
    return chamfer.Shape()
