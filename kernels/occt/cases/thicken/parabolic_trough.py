"""A parabolic trough, a B-spline sheet, thickened on its concave side. The parabolic cylinder
z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, a B-spline surface of degree 2 by 1, thickened by 1 toward
the parabola's focus."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp_Pnt

from thicken import thicken

SECTION = [(-4, 2), (0, -2), (4, 2)]  # (x, z) of the control points in u
YS = [0, 10]  # y of the control points in v
THICKNESS = 1


def array(kind, values):
    out = kind(1, len(values))
    for i, value in enumerate(values):
        out.SetValue(i + 1, value)
    return out


def build():
    poles = Array2_gp_Pnt(1, len(SECTION), 1, len(YS))
    for i, (x, z) in enumerate(SECTION):
        for j, y in enumerate(YS):
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, z))
    knots = array(Array1_double, [0, 1])
    surface = Geom_BSplineSurface(poles, knots, knots, array(Array1_int, [3, 3]),
                                  array(Array1_int, [2, 2]), 2, 1)
    # ∂S/∂u × ∂S/∂v is +z at the vertex: the normal points to the concave side
    sheet = BRepBuilderAPI_MakeFace(surface, 1e-9).Face()
    return thicken(sheet, THICKNESS)
