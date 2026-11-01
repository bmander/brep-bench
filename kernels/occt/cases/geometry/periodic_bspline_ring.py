"""A ring turned from a periodic B-spline. A ring about the z axis, its section a closed, periodic
B-spline curve turned a whole turn about the axis.

The section lies in the xz plane. It is the periodic (unclamped, closed) uniform quadratic B-spline,
non-rational, whose control polygon is the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3) and
(13, 0, −3), taken in that order and closing back on the first: a square of side 6 centred at (10,
0, 0), rounded all the way to the middle of each side. The curve is four parabolic arcs, each from
the middle of one side to the middle of the next ((13, 0, 0) to (10, 0, 3), and so on), tangent to
the sides there. It is smooth everywhere and has no end.

The ring's surface is that curve turned about the z axis: periodic both ways, round the axis and
round the section, so a kernel has a closed surface with no boundary at all and must place both of
its seams itself."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp, gp_Pnt

CORNERS = [(13, 3), (7, 3), (7, -3), (13, -3)]  # (x, z): the control polygon, closing on itself


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def section():
    """The periodic uniform quadratic B-spline on the square, in the xz plane: in OCCT a periodic
    curve has as many poles as the polygon has corners, and its knots (here 0, 1, 2, 3, 4) each
    once."""
    poles = array(Array1_gp_Pnt, [gp_Pnt(x, 0, z) for x, z in CORNERS])
    knots = array(Array1_double, list(range(len(CORNERS) + 1)))
    mults = array(Array1_int, [1] * (len(CORNERS) + 1))
    return Geom_BSplineCurve(poles, knots, mults, 2, True)


def build():
    edge = BRepBuilderAPI_MakeEdge(section()).Edge()
    disc = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(edge).Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(disc, gp.OZ_s()).Shape()
