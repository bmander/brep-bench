"""A cylinder whose rim is rounded into a dome. A cylinder of radius 5 on the z axis, from z = 0
to z = 10. Round the circular edge around its top face to radius 5, the cylinder's own radius.

The ball rolling round the rim has its centre on the axis, at (0, 0, 5), so the top face shrinks
to nothing and the round is a hemisphere of radius 5 centred there, tangent to the cylinder's
side along the circle at z = 5. The solid is a cylinder from z = 0 to z = 5 under that dome, its
top the point (0, 0, 10)."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

CYLINDER_RADIUS = 5
HEIGHT = 10
RADIUS = 5
TOLERANCE = 1e-7


def build():
    cylinder = BRepPrimAPI_MakeCylinder(CYLINDER_RADIUS, HEIGHT).Shape()
    fillet = BRepFilletAPI_MakeFillet(cylinder)
    # The rim is the circle at the top.
    edges = TopExp_Explorer(cylinder, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Location().Z() - HEIGHT) < TOLERANCE):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} circles at z = {HEIGHT}"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the rim to the cylinder's radius")
    return fillet.Shape()
