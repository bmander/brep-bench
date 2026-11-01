"""A block under a Bézier patch of degree 15. A block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20
of the xy plane, its floor flat at z = 0, its four sides upright in the planes x = 0, x = 20, y = 0
and y = 20, and its top a Bézier patch of degree 15 each way, non-rational: a net of 16 × 16 control
points.

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1 and i, j = 0 … 15, with the
Bernstein polynomials of degree 15, Bᵢ(t) = C(15, i) tⁱ (1 − t)¹⁵⁻ⁱ. The control point Pᵢⱼ is
(20i/15, 20j/15, zᵢⱼ), standing over an even 16 × 16 grid of the square, at the height zᵢⱼ = 4 +
((3i + 5j) mod 7): a field of heights from 4 to 10, never the same at two neighbours.

Because the control points' x and y lie on an even grid, x = 20u and y = 20v, so the patch is the
graph of a height over the square, everywhere at least 4 above the floor, and its four edges are the
upright sides' top edges: each side is the flat region between the floor and the curve of degree 15
the patch's edge draws in that side's plane."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierSurface
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

SIDE = 20
DEGREE = 15  # each way: a net of 16 × 16 control points


def height(i, j):
    return 4 + (3 * i + 5 * j) % 7


def dome():
    """The top: the Bézier patch of degree 15 each way over the square, as a face."""
    poles = Array2_gp_Pnt(1, DEGREE + 1, 1, DEGREE + 1)
    for i in range(DEGREE + 1):
        for j in range(DEGREE + 1):
            poles.SetValue(i + 1, j + 1,
                           gp_Pnt(SIDE * i / DEGREE, SIDE * j / DEGREE, height(i, j)))
    return BRepBuilderAPI_MakeFace(Geom_BezierSurface(poles), 1e-9).Face()


def build():
    # the top swept straight down past the floor, then cut off at the floor by a box wider than
    # the square
    column = BRepPrimAPI_MakePrism(dome(), gp_Vec(0, 0, -2 * SIDE)).Shape()
    floor_up = BRepPrimAPI_MakeBox(gp_Pnt(-1, -1, 0), gp_Pnt(SIDE + 1, SIDE + 1, 2 * SIDE)).Shape()
    block = BRepAlgoAPI_Common(column, floor_up)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the swept dome at the floor")
    return block.Shape()
