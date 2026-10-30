"""A block under a bicubic Bézier dome. A block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20
of the xy plane, its floor flat at z = 0, its four sides upright in the planes x = 0, x = 20,
y = 0 and y = 20, and its top a bicubic Bézier patch.

The patch's sixteen control points are Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ) for i, j = 0, 1, 2, 3, so that
they stand over an even 4 × 4 grid of the square (x = 0, 20/3, 40/3, 20 and the same for y), at
these heights zᵢⱼ (row i, x = 20i/3; column j, y = 20j/3): 4 6 7 5 / 6 14 12 7 / 5 13 16 8 /
3 6 8 6.

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1, with the cubic Bernstein
polynomials. Because the control points' x and y lie on an even grid, x = 20u and y = 20v, so the
patch is the graph of a height over the square, everywhere at least 3 above the floor, and its
four edges are the upright sides' top edges: each side is the flat region between the floor and
the cubic curve the patch's edge draws in that side's plane."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierSurface
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

SIDE = 20
HEIGHTS = [[4, 6, 7, 5],
           [6, 14, 12, 7],
           [5, 13, 16, 8],
           [3, 6, 8, 6]]  # row i at x = 20i/3, column j at y = 20j/3


def dome():
    """The top: the Bézier patch over the square, as a face."""
    poles = Array2_gp_Pnt(1, 4, 1, 4)
    for i, row in enumerate(HEIGHTS):
        for j, z in enumerate(row):
            poles.SetValue(i + 1, j + 1, gp_Pnt(SIDE * i / 3, SIDE * j / 3, z))
    return BRepBuilderAPI_MakeFace(Geom_BezierSurface(poles), 1e-9).Face()


def build():
    # the top swept straight down past the floor, its sides the upright planes through the
    # patch's edges, then cut off at the floor by a box wider than the square
    column = BRepPrimAPI_MakePrism(dome(), gp_Vec(0, 0, -2 * SIDE)).Shape()
    floor_up = BRepPrimAPI_MakeBox(gp_Pnt(-1, -1, 0), gp_Pnt(SIDE + 1, SIDE + 1, 2 * SIDE)).Shape()
    block = BRepAlgoAPI_Common(column, floor_up)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the swept dome at the floor")
    return block.Shape()
