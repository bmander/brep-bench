"""A drum under a saddle, bored through. The drum of `geometry/saddle-drum`, bored through along its
axis: every point with 9 ≤ x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8.

The saddle is the same B-spline surface: the bilinear patch of degree 1 each way, non-rational,
whose four control points are (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and (10, 10, 22.5).
The drum's top is the part of it between the circles of radius 3 and 8 about the z axis, a B-spline
face trimmed by an outer loop and with a hole. The bore, of radius 3, runs from the floor to the
saddle, which it meets at heights from 9.4375 to 10.5625. If you cut it with a cylinder, make it run
past the floor and the saddle."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

RADIUS, BORE = 8, 3
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
    column = BRepPrimAPI_MakePrism(saddle(), gp_Vec(0, 0, -40)).Shape()
    drum = BRepAlgoAPI_Common(column, BRepPrimAPI_MakeCylinder(RADIUS, 30).Shape())
    if not drum.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to trim the saddle by the drum")
    # the bore, from below the floor to above the saddle
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), BORE, 30).Shape()
    bored = BRepAlgoAPI_Cut(drum.Shape(), drill)
    if not bored.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to bore the drum")
    return bored.Shape()
