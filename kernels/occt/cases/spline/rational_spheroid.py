"""A spheroid turned from a weighted quadratic B-spline. A solid made by turning a plane region a
full turn about the z axis. The region lies in the xz plane, on the side x ≥ 0, between the
segment of the z axis from (0, 0, −10) to (0, 0, 10) and a rational quadratic B-spline curve
joining the same two points.

The curve has five control points, (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10) and
(0, 0, 10), with weights 1, √2/2, 1, √2/2 and 1, and the clamped knot vector
(0, 0, 0, ½, ½, 1, 1, 1). The double knot at ½ splits it into two rational quadratic Bézier
segments, each through three of the control points (the middle one, (6, 0, 0), shared). Each
segment is a quarter of the ellipse x²/36 + z²/100 = 1: the curve leaves the axis at the bottom
square to it, swells to x = 6 at z = 0 and returns to the axis square to it at the top. Without
the weights the segments would be parabolas.

So the solid is a smooth egg-like spheroid, 12 wide and 20 tall, with one face whose two poles
lie on the axis."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp, gp_Pnt

from bench import Refused

CONTROL = [(0, 0, -10), (6, 0, -10), (6, 0, 0), (6, 0, 10), (0, 0, 10)]
WEIGHTS = [1, math.sqrt(2) / 2, 1, math.sqrt(2) / 2, 1]
KNOTS, MULTS = [0, 0.5, 1], [3, 2, 3]  # the knot vector (0, 0, 0, ½, ½, 1, 1, 1)
DEGREE = 2


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def build():
    spline = Geom_BSplineCurve(array(Array1_gp_Pnt, [gp_Pnt(*p) for p in CONTROL]),
                               array(Array1_double, WEIGHTS), array(Array1_double, KNOTS),
                               array(Array1_int, MULTS), DEGREE)
    bottom, top = gp_Pnt(*CONTROL[0]), gp_Pnt(*CONTROL[-1])
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(spline).Edge(),
                                   BRepBuilderAPI_MakeEdge(top, bottom).Edge())
    if not wire.IsDone():
        raise Refused("the profile's two edges did not join into a wire")
    region = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(region, gp.OZ_s(), 2 * math.pi).Shape()
