"""A cone bounded by a NURBS surface with a pole. A solid cone standing on the disc of radius 5
about the z axis in the plane z = 0, its apex at (0, 0, 10), whose side is one rational B-spline
surface (a NURBS) with a pole at the apex, closed by a flat disc at its base.

The side is rational quadratic round the axis (u) and linear up it (v): a 9 × 2 net. Its first row
(v = 0) is the base circle of radius 5 as a rational quadratic: the nine control points 5cᵢ in the
plane z = 0, cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1), (1, −1), (1, 0), with
weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1,
1). Its second row (v = 1) is nine copies of the apex (0, 0, 10), with the same weights wᵢ, and v
runs on the knots (0, 0, 1, 1). Each iso-u line is the straight line from the circle to the apex,
and the whole edge v = 1 of the surface collapses to the apex.

Build the cone from that face and the base's disc; do not use a cone."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BSplineSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp, gp_Circ, gp_Pnt

from bench import Refused

R, HEIGHT = 5, 10
H = math.sqrt(2) / 2
CIRCLE = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
CIRCLE_W = [1, H, 1, H, 1, H, 1, H, 1]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def side():
    """The 9 × 2 net: the base circle, then nine copies of the apex, each column's weight its
    circle pole's."""
    poles = Array2_gp_Pnt(1, len(CIRCLE), 1, 2)
    weights = Array2_double(1, len(CIRCLE), 1, 2)
    for i, ((cx, cy), w) in enumerate(zip(CIRCLE, CIRCLE_W)):
        poles.SetValue(i + 1, 1, gp_Pnt(R * cx, R * cy, 0))
        poles.SetValue(i + 1, 2, gp_Pnt(0, 0, HEIGHT))
        weights.SetValue(i + 1, 1, w)
        weights.SetValue(i + 1, 2, w)
    surface = Geom_BSplineSurface(poles, weights,
                                  array(Array1_double, [0, 0.25, 0.5, 0.75, 1]),
                                  array(Array1_double, [0, 1]),
                                  array(Array1_int, [3, 2, 2, 2, 3]), array(Array1_int, [2, 2]),
                                  2, 1)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def build():
    rim = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), R)).Edge()).Wire()
    base = BRepBuilderAPI_MakeFace(rim).Face()
    sewing = BRepBuilderAPI_Sewing(1e-7)
    for face in (side(), base):
        sewing.Add(face)
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the side and its base")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the sewn shell")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid
