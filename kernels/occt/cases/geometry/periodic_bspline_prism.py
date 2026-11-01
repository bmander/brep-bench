"""A prism on a periodic B-spline. A prism standing on the xy plane, from z = 0 to z = 10, whose
section is a closed, periodic B-spline curve: a square with rounded corners, rounded all the way to
the middle of each side.

The curve is the periodic (unclamped, closed) uniform quadratic B-spline, non-rational, whose
control polygon is the square of corners (10, 10), (−10, 10), (−10, −10) and (10, −10) (in the plane
z = 0, taken in that order and closing back on the first). Its knots are evenly spaced and it has no
end: it is one smooth curve, continuous in its tangent everywhere, made of four parabolic arcs, each
running from the middle of one side to the middle of the next, (10, 0) to (0, 10) and so on, tangent
to the sides there, with the square's corner as its control point.

The prism's side is that curve swept straight up: a periodic B-spline surface, closed round on
itself with no seam a kernel must place anywhere in particular. Its ends are flat."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Pnt, gp_Vec

CORNERS = [(10, 10), (-10, 10), (-10, -10), (10, -10)]  # the control polygon, closing on itself
HEIGHT = 10


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def section():
    """The periodic uniform quadratic B-spline on the square: in OCCT a periodic curve has as many
    poles as the polygon has corners, and its knots (here 0, 1, 2, 3, 4) each once."""
    poles = array(Array1_gp_Pnt, [gp_Pnt(x, y, 0) for x, y in CORNERS])
    knots = array(Array1_double, list(range(len(CORNERS) + 1)))
    mults = array(Array1_int, [1] * (len(CORNERS) + 1))
    return Geom_BSplineCurve(poles, knots, mults, 2, True)


def build():
    edge = BRepBuilderAPI_MakeEdge(section()).Edge()
    end = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(edge).Wire()).Face()
    return BRepPrimAPI_MakePrism(end, gp_Vec(0, 0, HEIGHT)).Shape()
