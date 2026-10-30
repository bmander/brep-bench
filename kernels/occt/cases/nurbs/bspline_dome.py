"""A block under a bicubic B-spline with a double knot. A block standing on the square
0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0, its four sides upright in the
planes x = 0, x = 20, y = 0 and y = 20, and its top a clamped bicubic B-spline surface,
non-rational, of 6 × 5 control points.

The surface is S(u, v) = Σᵢ Σⱼ Nᵢ(u) Mⱼ(v) Pᵢⱼ for u, v from 0 to 1, where Nᵢ (i = 0 … 5) are the
cubic B-spline basis functions on the knot vector (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1), with a
double knot at 2/5, and Mⱼ (j = 0 … 4) the cubic ones on (0, 0, 0, 0, 1/3, 1, 1, 1, 1). The control
point Pᵢⱼ = (xᵢ, yⱼ, zᵢⱼ) has x₀ … x₅ = 0, 3, 7, 11, 16, 20 and y₀ … y₄ = 0, 5, 10, 15, 20, and
these heights zᵢⱼ (row i, column j): 3 5 6 5 4 / 5 9 11 8 5 / 6 12 15 10 6 / 4 10 18 9 5 /
5 8 9 7 4 / 2 4 5 4 3.

Because every control point of row i has the same x and every one of column j the same y, the
surface's x depends on u alone and its y on v alone, each rising steadily from 0 to 20 (but not
in proportion to u or v: the control points are not at the knots' averages). So the surface is
the graph of a height over the square, everywhere at least 2 above the floor, and its four edges
lie in the four sides' planes: each side is the flat region between the floor and the cubic
B-spline curve the surface's edge draws in that plane. The double knot leaves the surface only
C¹ across u = 2/5: its curvature jumps along that line."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

SIDE = 20
XS = [0, 3, 7, 11, 16, 20]  # the x of each row of control points
YS = [0, 5, 10, 15, 20]     # the y of each column
HEIGHTS = [[3, 5, 6, 5, 4],
           [5, 9, 11, 8, 5],
           [6, 12, 15, 10, 6],
           [4, 10, 18, 9, 5],
           [5, 8, 9, 7, 4],
           [2, 4, 5, 4, 3]]
U_KNOTS, U_MULTS = [0, 2 / 5, 1], [4, 2, 4]  # (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1)
V_KNOTS, V_MULTS = [0, 1 / 3, 1], [4, 1, 4]  # (0, 0, 0, 0, 1/3, 1, 1, 1, 1)
DEGREE = 3


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def top():
    poles = Array2_gp_Pnt(1, len(XS), 1, len(YS))
    for i, x in enumerate(XS):
        for j, y in enumerate(YS):
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, HEIGHTS[i][j]))
    surface = Geom_BSplineSurface(poles, array(Array1_double, U_KNOTS),
                                  array(Array1_double, V_KNOTS), array(Array1_int, U_MULTS),
                                  array(Array1_int, V_MULTS), DEGREE, DEGREE)
    return BRepBuilderAPI_MakeFace(surface, 1e-9).Face()


def build():
    # the top swept straight down past the floor, then cut off at the floor
    column = BRepPrimAPI_MakePrism(top(), gp_Vec(0, 0, -2 * SIDE)).Shape()
    floor_up = BRepPrimAPI_MakeBox(gp_Pnt(-1, -1, 0), gp_Pnt(SIDE + 1, SIDE + 1, 2 * SIDE)).Shape()
    block = BRepAlgoAPI_Common(column, floor_up)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the swept surface at the floor")
    return block.Shape()
