"""A cylinder whose rim is chamfered to a point. A cylinder of radius 5 on the z axis, from
z = 0 to z = 10. Chamfer the circular edge around its top face with the same distance 5 on both
faces, the cylinder's own radius: 5 in from the rim across the top and 5 down the side.

The top face shrinks to its centre, and the chamfer is the cone from the circle of radius 5 at
z = 5 up to its apex at (0, 0, 10), its half-angle 45°. The solid is a cylinder from z = 0 to
z = 5 under that cone."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

CYLINDER_RADIUS = 5
HEIGHT = 10
DISTANCE = 5
TOLERANCE = 1e-7


def build():
    cylinder = BRepPrimAPI_MakeCylinder(CYLINDER_RADIUS, HEIGHT).Shape()
    chamfer = BRepFilletAPI_MakeChamfer(cylinder)
    # The rim is the circle at the top.
    edges = TopExp_Explorer(cylinder, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Location().Z() - HEIGHT) < TOLERANCE):
            chamfer.Add(DISTANCE, edge)
        edges.Next()
    assert chamfer.NbContours() == 1, f"{chamfer.NbContours()} circles at z = {HEIGHT}"
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the rim to the cylinder's radius")
    return chamfer.Shape()
