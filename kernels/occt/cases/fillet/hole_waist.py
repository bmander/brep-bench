"""A hole whose two rim rounds meet at its waist. A square plate from (−10, −10, 0) to
(10, 10, 2), with a hole of radius 3 through it on the z axis. Round both circular edges of the
hole, the rim on the top face and the rim on the bottom face, each to radius 1: half the plate's
thickness.

Each round is a quarter of the torus about the z axis whose centre circle has radius 4 at z = 1
and whose tube has radius 1, so the hole's cylindrical wall vanishes entirely: the two rounds
meet on the waist circle of radius 3 at z = 1, tangent to each other there, and the hole's whole
surface is the inner half of that torus. The rounds meet the top and bottom faces on the circles
of radius 4."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

HALF_SIDE, THICKNESS = 10, 2
HOLE_RADIUS = 3
RADIUS = 1
TOLERANCE = 1e-7


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    # The drill runs 1 past both faces of the plate.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()),
                                     HOLE_RADIUS, THICKNESS + 2).Shape()
    cut = BRepAlgoAPI_Cut(plate, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    part = cut.Shape()

    fillet = BRepFilletAPI_MakeFillet(part)
    # The hole's two rims: the circles of the hole's radius.
    edges = TopExp_Explorer(part, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Radius() - HOLE_RADIUS) < TOLERANCE):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 2, f"{fillet.NbContours()} rims of radius {HOLE_RADIUS}"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the hole's two rims")
    return fillet.Shape()
