"""A ball's cap turned from a weighted quadratic arc. A solid made by turning a plane region a
full turn about the z axis. The region lies in the xz plane, on the side x ≥ 0, and is bounded by
three pieces: the segment of the z axis from (0, 0, −5) to (0, 0, 10); the segment from
(0, 0, −5) to (5√3, 0, −5); and a rational quadratic Bézier curve, one segment, from
(5√3, 0, −5) to (0, 0, 10).

The curve's three control points are P0 = (5√3, 0, −5), P1 = (10√3, 0, 10) and
P2 = (0, 0, 10), with weights 1, ½ and 1: the point at parameter t is
(B0·P0 + ½B1·P1 + B2·P2) / (B0 + ½B1 + B2), where B0 = (1 − t)², B1 = 2t(1 − t) and B2 = t². The
weights matter: with them the curve is exactly the arc of the circle of radius 10 about the
origin, turning 120° from (5√3, 0, −5) over its widest point (10, 0, 0) to the top of the circle;
without them it would be a parabola.

So the solid stands on a flat disc of radius 5√3 at z = −5, bulges out to radius 10 at z = 0
and closes over the top at (0, 0, 10), where the curve meets the axis square to it."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_double, Array1_gp_Pnt
from OCP.gp import gp, gp_Pnt

from bench import Refused

S3 = math.sqrt(3)
CONTROL = [(5 * S3, 0, -5), (10 * S3, 0, 10), (0, 0, 10)]
WEIGHTS = [1, 0.5, 1]
FOOT = (0, 0, -5)  # where the base meets the axis


def rational_bezier(points, weights):
    poles, ws = Array1_gp_Pnt(1, len(points)), Array1_double(1, len(points))
    for i, (p, w) in enumerate(zip(points, weights)):
        poles.SetValue(i + 1, gp_Pnt(*p))
        ws.SetValue(i + 1, w)
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles, ws)).Edge()


def build():
    foot, rim, top = gp_Pnt(*FOOT), gp_Pnt(*CONTROL[0]), gp_Pnt(*CONTROL[-1])
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(foot, rim).Edge(),
                                   rational_bezier(CONTROL, WEIGHTS),
                                   BRepBuilderAPI_MakeEdge(top, foot).Edge())
    if not wire.IsDone():
        raise Refused("the profile's three edges did not join into a wire")
    region = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(region, gp.OZ_s(), 2 * math.pi).Shape()
