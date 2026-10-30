"""The Bézier dome drilled through off centre. The block under the bicubic Bézier dome of
`nurbs/bezier-dome`, with a round hole drilled straight down through it.

The block stands on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20, its floor flat at z = 0, its four sides
upright, and its top the bicubic Bézier patch whose sixteen control points are
Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ) for i, j = 0 … 3, at these heights (row i, x = 20i/3; column j,
y = 20j/3): 4 6 7 5 / 6 14 12 7 / 5 13 16 8 / 3 6 8 6. (S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ with the
cubic Bernstein polynomials; x = 20u and y = 20v, so the patch is the graph of a height over the
square.)

The hole is the vertical cylinder of radius 3 about the line x = 12, y = 8: it runs from the
floor right up through the dome, so it opens in the floor and in the domed top, where its rim is
a space curve, and stays clear of the four sides. If you drill it with a cylinder, make the
cylinder run past the floor and past the top."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierSurface
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp, gp_Ax2, gp_Pnt, gp_Vec

from bench import Refused

SIDE = 20
HEIGHTS = [[4, 6, 7, 5],
           [6, 14, 12, 7],
           [5, 13, 16, 8],
           [3, 6, 8, 6]]  # row i at x = 20i/3, column j at y = 20j/3
HOLE_AT = (12, 8)
HOLE_RADIUS = 3


def dome_block():
    """The block under the Bézier patch: the patch swept down past the floor, cut at the floor."""
    poles = Array2_gp_Pnt(1, 4, 1, 4)
    for i, row in enumerate(HEIGHTS):
        for j, z in enumerate(row):
            poles.SetValue(i + 1, j + 1, gp_Pnt(SIDE * i / 3, SIDE * j / 3, z))
    top = BRepBuilderAPI_MakeFace(Geom_BezierSurface(poles), 1e-9).Face()
    column = BRepPrimAPI_MakePrism(top, gp_Vec(0, 0, -2 * SIDE)).Shape()
    floor_up = BRepPrimAPI_MakeBox(gp_Pnt(-1, -1, 0), gp_Pnt(SIDE + 1, SIDE + 1, 2 * SIDE)).Shape()
    block = BRepAlgoAPI_Common(column, floor_up)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the swept dome at the floor")
    return block.Shape()


def build():
    # the drill from below the floor to above the dome's highest control point
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*HOLE_AT, -5), gp.DZ_s()), HOLE_RADIUS,
                                     2 * SIDE).Shape()
    drilled = BRepAlgoAPI_Cut(dome_block(), drill)
    if not drilled.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return drilled.Shape()
