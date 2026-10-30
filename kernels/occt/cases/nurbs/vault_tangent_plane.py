"""A weighted barrel vault cut by the plane along its crown. A block with a barrel-vaulted top,
from which everything above the plane z = 16 is cut away. That plane touches the vault along its
crown, so the cut removes nothing and the solid is the whole block; the cut is the case.

The block stands on the square −12 ≤ x ≤ 12, −12 ≤ y ≤ 12, its floor flat at z = 0 and its four
sides upright. Its top is a rational B-spline surface, of degree 1 along x and 2 along y,
non-periodic, with knot vectors (0, 0, 1, 1) along x and (0, 0, 0, 1, 1, 1) along y. Its six
control points are, for x = −12 and again for x = 12, the points (x, −12, 10), (x, 0, 26) and
(x, 12, 10), with weights 1, 3/5 and 1 in that order (so along y each row is one rational
quadratic Bézier arc). With these weights the top is exactly part of a circular cylinder: the
points with y² + (z − 1)² = 15², z ≥ 10, an arch of radius 15 about the line y = 0, z = 1, rising
from z = 10 at the sides y = ±12 to its crown z = 16 along the line y = 0. So the sides y = ±12
are rectangles 24 × 10, and the sides x = ±12 are arched.

Remove from the block everything with z > 16, for example by cutting it with a box whose floor is
the plane z = 16 and which reaches past the block on every other side. The box's floor touches
the vault along the whole crown line, from (−12, 0, 16) to (12, 0, 16), tangent to it there, and
meets the block nowhere else."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

HALF = 12                              # the square is −12 ≤ x, y ≤ 12
ARCH = [(-12, 10), (0, 26), (12, 10)]  # (y, z) of each control point along y
WEIGHTS = [1, 3 / 5, 1]
CROWN = 16                             # the cutting plane z = 16


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def vault():
    """The block: its weighted top swept down past the floor, cut at the floor."""
    poles, weights = Array2_gp_Pnt(1, 2, 1, 3), Array2_double(1, 2, 1, 3)
    for i, x in enumerate([-HALF, HALF]):
        for j, ((y, z), w) in enumerate(zip(ARCH, WEIGHTS)):
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, z))
            weights.SetValue(i + 1, j + 1, w)
    top = Geom_BSplineSurface(poles, weights, array(Array1_double, [0, 1]),
                              array(Array1_double, [0, 1]), array(Array1_int, [2, 2]),
                              array(Array1_int, [3, 3]), 1, 2)
    column = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(top, 1e-9).Face(),
                                   gp_Vec(0, 0, -100)).Shape()
    floor_up = BRepPrimAPI_MakeBox(gp_Pnt(-HALF - 1, -HALF - 1, 0),
                                   gp_Pnt(HALF + 1, HALF + 1, 50)).Shape()
    block = BRepAlgoAPI_Common(column, floor_up)
    if not block.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the swept vault at the floor")
    return block.Shape()


def build():
    above = BRepPrimAPI_MakeBox(gp_Pnt(-2 * HALF, -2 * HALF, CROWN),
                                gp_Pnt(2 * HALF, 2 * HALF, CROWN + 2 * HALF)).Shape()
    cut = BRepAlgoAPI_Cut(vault(), above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the tangent plane")
    return cut.Shape()
