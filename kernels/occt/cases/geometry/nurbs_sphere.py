"""A ball bounded by one NURBS surface with two poles. A ball of radius 5 centred on the origin, its
whole surface one rational B-spline surface (a NURBS), the standard biquadratic sphere: a 9 × 5 net
whose first and last rows each collapse to a single point, a pole.

The surface is the tensor product of two rational quadratic curves. Round the axis (u), the unit
circle of nine control points cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1), (1,
−1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ¼, ¼,
½, ½, ¾, ¾, 1, 1, 1). Along the meridian (v), the semicircle from the south pole to the north of
five control points (ρⱼ, zⱼ) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights vⱼ = 1, √2/2,
1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1). The net's control point (i, j) is (ρⱼ·cᵢ, zⱼ) =
(ρⱼ cᵢₓ, ρⱼ cᵢᵧ, zⱼ), with weight wᵢvⱼ.

So every control point with j = 0 is (0, 0, −5) and every one with j = 4 is (0, 0, 5): the surface's
whole edge v = 0 is the south pole, and its edge v = 1 the north. Its edges u = 0 and u = 1 are the
same meridian, in the half plane y = 0, x ≥ 0, which is the surface's seam. Build the ball from that
one face, closed on its seam and at its poles; do not use a sphere."""
import math

from OCP.BRep import BRep_Builder
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BSplineSurface
from OCP.TopoDS import TopoDS_Shell, TopoDS_Solid
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp_Pnt

R = 5
H = math.sqrt(2) / 2
CIRCLE = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0)]
CIRCLE_W = [1, H, 1, H, 1, H, 1, H, 1]
MERIDIAN = [(0, -R), (R, -R), (R, 0), (R, R), (0, R)]   # (ρ, z), south pole to north
MERIDIAN_W = [1, H, 1, H, 1]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def surface():
    poles = Array2_gp_Pnt(1, len(CIRCLE), 1, len(MERIDIAN))
    weights = Array2_double(1, len(CIRCLE), 1, len(MERIDIAN))
    for i, ((cx, cy), wi) in enumerate(zip(CIRCLE, CIRCLE_W)):
        for j, ((rho, z), wj) in enumerate(zip(MERIDIAN, MERIDIAN_W)):
            poles.SetValue(i + 1, j + 1, gp_Pnt(rho * cx, rho * cy, z))
            weights.SetValue(i + 1, j + 1, wi * wj)
    return Geom_BSplineSurface(poles, weights,
                               array(Array1_double, [0, 0.25, 0.5, 0.75, 1]),
                               array(Array1_double, [0, 0.5, 1]),
                               array(Array1_int, [3, 2, 2, 2, 3]), array(Array1_int, [3, 2, 3]),
                               2, 2)


def build():
    # OCCT makes the face's boundary itself: the surface is closed round u, so its edges u = 0 and
    # u = 1 are one seam, and its edges v = 0 and v = 1, each one point, are degenerate edges at
    # the poles. That face alone is a closed shell, and the shell bounds the ball.
    face = BRepBuilderAPI_MakeFace(surface(), 1e-9).Face()
    builder = BRep_Builder()
    shell, solid = TopoDS_Shell(), TopoDS_Solid()
    builder.MakeShell(shell)
    builder.Add(shell, face)
    shell.Closed(True)
    builder.MakeSolid(solid)
    builder.Add(solid, shell)
    BRepLib.OrientClosedSolid_s(solid)
    return solid
