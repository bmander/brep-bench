"""A drum under a saddle. A round drum of radius 8 about the z axis, its floor flat at z = 0 and its
top the saddle z = 10 + xy/8: every point with x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8.

Make the saddle as a B-spline surface: the bilinear patch of degree 1 each way, non-rational, whose
four control points are (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and (10, 10, 22.5), the
corners of the square −10 ≤ x, y ≤ 10 at the heights of the saddle there. That patch is exactly the
saddle over the whole square (a bilinear patch is a hyperbolic paraboloid). The drum's top is the
part of it inside the circle of radius 8, trimmed by the drum's side: it rises to 14 above the
diagonal x = y and falls to 6 above x = −y."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

RADIUS = 8
# the saddle z = 10 + xy/8 at the corners of the square −10 ≤ x, y ≤ 10
CORNERS = [[(-10, -10, 22.5), (-10, 10, -2.5)],
           [(10, -10, -2.5), (10, 10, 22.5)]]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def saddle():
    """The bilinear B-spline patch, degree 1 each way, as a face over the whole square."""
    poles = Array2_gp_Pnt(1, 2, 1, 2)
    for i in range(2):
        for j in range(2):
            poles.SetValue(i + 1, j + 1, gp_Pnt(*CORNERS[i][j]))
    knots, mults = array(Array1_double, [0, 1]), array(Array1_int, [2, 2])
    surface = Geom_BSplineSurface(poles, knots, knots, mults, mults, 1, 1)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def build():
    # the saddle swept straight down past the floor, then in common with the drum's cylinder from
    # the floor up past the saddle: the cylinder trims the patch to the drum's top
    column = BRepPrimAPI_MakePrism(saddle(), gp_Vec(0, 0, -40)).Shape()
    drum = BRepPrimAPI_MakeCylinder(RADIUS, 30).Shape()
    block = BRepAlgoAPI_Common(column, drum)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to trim the saddle by the drum")
    return block.Shape()
