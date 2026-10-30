"""A peanut with a thin waist, a periodic spline extruded. A prism 10 tall standing on the xy
plane, from z = 0 to z = 10, whose section is a peanut: the region inside a closed, uniform,
periodic cubic B-spline curve in the plane z = 0.

The curve's twelve control points, in order (counter-clockwise round the peanut), are
(14, 0), (12, 8), (4, 6), (0, −21/8), (−4, 6), (−12, 8), (−14, 0), (−12, −8), (−4, −6),
(0, 21/8), (4, −6) and (12, −8), and after the twelfth the curve comes back to the first. Its
knots are uniform and it is periodic, so the curve is C² everywhere, with no start or end: each
of its twelve spans is the uniform cubic B-spline segment of four consecutive control points
(wrapping round), the span from Pₖ, Pₖ₊₁, Pₖ₊₂, Pₖ₊₃ being
((1 − t)³Pₖ + (3t³ − 6t² + 4)Pₖ₊₁ + (−3t³ + 3t² + 3t + 1)Pₖ₊₂ + t³Pₖ₊₃) / 6 for t from 0 to 1.

The two waist control points lie on the far side of the x axis from the boundary they pull on
(the control polygon crosses itself there), so the curve itself does not cross: the peanut's two
lobes, about 28 across in all and 12 tall, are joined by a waist only 0.5 wide, the curve
passing through (0, 0.25) and (0, −0.25). The section is one region, and the prism's side is a
single smooth face round it."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

CONTROL = [(14, 0), (12, 8), (4, 6), (0, -21 / 8), (-4, 6), (-12, 8), (-14, 0), (-12, -8),
           (-4, -6), (0, 21 / 8), (4, -6), (12, -8)]
DEGREE = 3
HEIGHT = 10


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def build():
    # a periodic B-spline: one knot more than poles, each of multiplicity 1, evenly spaced
    n = len(CONTROL)
    curve = Geom_BSplineCurve(array(Array1_gp_Pnt, [gp_Pnt(x, y, 0) for x, y in CONTROL]),
                              array(Array1_double, list(range(n + 1))),
                              array(Array1_int, [1] * (n + 1)), DEGREE, True)
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(curve).Edge())
    if not wire.IsDone():
        raise Refused("the closed spline did not make a wire")
    section = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not section.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace could not fill the peanut")
    return BRepPrimAPI_MakePrism(section.Face(), gp_Vec(0, 0, HEIGHT)).Shape()
