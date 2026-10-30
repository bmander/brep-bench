"""A cube edge rounded with a radius growing along it. A cube of side 10 between the origin and
(10, 10, 10). Round the edge where its faces x = 10 and y = 10 meet, the line from (10, 10, 0) to
(10, 10, 10), with a radius that grows linearly along it: r = 1 + z/10, so 1 at the bottom face
z = 0 and 2 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by
the horizontal plane is the quarter circle of radius r(z) centred at (10 − r, 10 − r, z), tangent
to the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). (So the round is
part of an oblique circular cone with its apex at (10, 10, −10), tangent to both faces along
straight lines.) It cuts the bottom face with the quarter circle of radius 1 and the top face
with the quarter circle of radius 2."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

SIDE = 10
RADIUS_BOTTOM, RADIUS_TOP = 1, 2  # at z = 0 and z = 10
TOLERANCE = 1e-7


def on_both_faces(point):
    """Whether a point lies on the face x = 10 and on the face y = 10."""
    return abs(point.X() - SIDE) < TOLERANCE and abs(point.Y() - SIDE) < TOLERANCE


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    fillet = BRepFilletAPI_MakeFillet(cube)
    edges = TopExp_Explorer(cube, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        first = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        last = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if on_both_faces(first) and on_both_faces(last):
            # A linear law from the spine's first vertex to its last: give each end its radius.
            if first.Z() < last.Z():
                fillet.Add(RADIUS_BOTTOM, RADIUS_TOP, edge)
            else:
                fillet.Add(RADIUS_TOP, RADIUS_BOTTOM, edge)
            break
        edges.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} edges at x = y = {SIDE}"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge with a growing radius")
    return fillet.Shape()
