"""A cylinder with its top rim rounded. A cylinder of radius 5 on the z axis, from z = 0 to
z = 10. Round the circular edge around its top face to radius 1."""
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
RADIUS = 1
TOLERANCE = 1e-7


def build():
    cylinder = BRepPrimAPI_MakeCylinder(CYLINDER_RADIUS, HEIGHT).Shape()
    fillet = BRepFilletAPI_MakeFillet(cylinder)
    # The rim is the circle at the top: an edge whose curve is a circle centred at z = 10.
    # (The other edges are the bottom circle and the seam, a straight line.)
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
        raise Refused("BRepFilletAPI_MakeFillet could not round the rim")
    return fillet.Shape()
