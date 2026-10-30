"""The Bézier dome sliced by a plane through a corner. The block under the bicubic Bézier dome of
`nurbs/bezier-dome`, with everything above the inclined plane z = 4 + x/2 cut away.

The block stands on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20, its floor flat at z = 0, its four sides
upright, and its top the bicubic Bézier patch whose sixteen control points are
Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ) for i, j = 0 … 3, at these heights (row i, x = 20i/3; column j,
y = 20j/3): 4 6 7 5 / 6 14 12 7 / 5 13 16 8 / 3 6 8 6. (S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ with the
cubic Bernstein polynomials; x = 20u and y = 20v, so the patch is the graph of a height over the
square.)

The solid is the part of that block on or below the plane z = 4 + x/2, which rises along x and is
level along y. The plane passes exactly through the block's corner (0, 0, 4), where the dome
meets the sides x = 0 and y = 0: the dome's height there is 4, its control point P₀₀. From that
corner the line where the plane meets the dome runs out across the top, leaving the corner along
(3, 2) in plan, to the side y = 20, which it reaches near x = 4.23. On the side x = 0 the plane
cuts the side down to the rectangle 0 ≤ z ≤ 4, touching the dome's edge there only at the
corner. Beyond the line, towards larger x, the dome lies below the plane and is kept; between
the line and the side x = 0 the top is the flat cut."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BezierSurface
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

SIDE = 20
HEIGHTS = [[4, 6, 7, 5],
           [6, 14, 12, 7],
           [5, 13, 16, 8],
           [3, 6, 8, 6]]  # row i at x = 20i/3, column j at y = 20j/3
CORNER = (0, 0, 4)         # the plane z = 4 + x/2 passes through it
SLOPE = 0.5                # dz/dx of the plane


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
    # a box standing on the plane, its floor in the plane and reaching well past the block
    normal = gp_Dir(-SLOPE, 0, 1)
    along = gp_Dir(1, 0, SLOPE)  # level along y, rising along x: in the plane
    reach = 3 * SIDE
    corner = gp_Pnt(*CORNER).Translated(gp_Vec(along).Multiplied(-reach / 2)
                                        .Added(gp_Vec(0, -reach / 2, 0)))
    above = BRepPrimAPI_MakeBox(gp_Ax2(corner, normal, along), reach, reach, reach).Shape()
    sliced = BRepAlgoAPI_Cut(dome_block(), above)
    if not sliced.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slice")
    return sliced.Shape()
