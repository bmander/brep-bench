"""An octahedron with every edge chamfered, its corners mitred. The octahedron |x| + |y| + |z| ≤ 10: its six corners on the axes, 10 from the origin, such as
(10, 0, 0), and its eight faces equilateral triangles, such as the one in the plane
x + y + z = 10. Chamfer all twelve of its edges, each with the same distance √6 on both of its
faces, measured across the face square to the edge: the chamfer of the edge from (10, 0, 0) to
(0, 10, 0) is the plane x + y = 8, through the two lines, one on each of its faces, at distance √6
from the edge (on the face x + y + z = 10, the line z = 2). No face closes a corner: each chamfer
runs on until it meets the others, so the four at each corner meet in a point, the corner mitred.

The solid is the octahedron cut by the twelve chamfer planes, |x| + |y| ≤ 8, |y| + |z| ≤ 8 and
|z| + |x| ≤ 8; `chamfer/octahedron-all-edges` is this solid cut again by a plane at each corner.
It has 20 flat faces. Each face of the octahedron becomes the equilateral triangle of side 4√2
inset from its edges, for the face x + y + z = 10 the triangle through (6, 2, 2), (2, 6, 2) and
(2, 2, 6). Each chamfer is a hexagon, a rectangle 4√2 long and 4 wide with a triangle at each end:
the chamfer x + y = 8 has corners (8, 0, 0), (6, 2, 2), (2, 6, 2), (0, 8, 0), (2, 6, −2) and
(6, 2, −2). The four chamfers at the corner (10, 0, 0) meet at the point (8, 0, 0).

It is what chamfering the edges one after another leaves, each edge its own chamfer, ending on
the faces at its ends, the chamfers made before it among them: build it so, with the kernel's
chamfer, in any order."""
from itertools import product

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

SIZE = 10
# the chamfers' planes, |x| + |y| = 8 and the others: distance √6 across a face drops x + y by 2
CHAMFER = SIZE - 2


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
    # BRepFilletAPI_MakeChamfer of all twelve edges at once declines the corners where four meet
    # (chamfer/octahedron-all-edges), and the edges chamfered one at a time, as the cube's mitre is
    # (chamfer/cube-all-edges-mitred), leave a solid BRepCheck_Analyzer calls invalid, its volume
    # wrong in some orders; so each chamfer is cut as the half-space beyond its plane.
    solid = octahedron()
    for u, v in ((0, 1), (1, 2), (2, 0)):
        for su, sv in product((1, -1), repeat=2):
            normal = [0.0, 0.0, 0.0]
            normal[u], normal[v] = su, sv
            point = [0.0, 0.0, 0.0]
            point[u] = su * CHAMFER
            plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(*point), gp_Dir(*normal))).Face()
            beyond = [p + 2 * SIZE * n for p, n in zip(point, normal)]
            cut = BRepAlgoAPI_Cut(solid, BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(*beyond)).Solid())
            if not cut.IsDone():
                raise Refused("BRepAlgoAPI_Cut declined a chamfer's half-space")
            solid = cut.Shape()
    return solid
