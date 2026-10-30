"""A groin vault: two weighted barrel vaults in common. The common part of two blocks with
barrel-vaulted tops, the second the first turned a quarter turn about the z axis, so that the
two vaults cross at right angles: a groin vault.

The first block stands on the square −12 ≤ x ≤ 12, −12 ≤ y ≤ 12, its floor flat at z = 0 and its
four sides upright. Its top is a rational B-spline surface, of degree 1 along x and 2 along y,
non-periodic, with knot vectors (0, 0, 1, 1) along x and (0, 0, 0, 1, 1, 1) along y. Its six
control points are, for x = −12 and again for x = 12, the points (x, −12, 10), (x, 0, 26) and
(x, 12, 10), with weights 1, 3/5 and 1 in that order (so along y each row is one rational
quadratic Bézier arc). With these weights the top is exactly part of a circular cylinder: the
points with y² + (z − 1)² = 15², z ≥ 10, an arch of radius 15 about the line y = 0, z = 1, rising
from z = 10 at the sides y = ±12 to its crown z = 16 along y = 0. (Without the weights it would be
a parabolic vault.) So the sides y = ±12 are rectangles 24 × 10, and the sides x = ±12 are arched.

The second block is the first turned 90° about the z axis: its arch runs along y, rising from
z = 10 at x = ±12 to its crown along x = 0.

Their common part keeps, over every point of the square, the lower of the two vaults. The two
vaults meet along the square's diagonals, in two arches (the groins), each an arc of an ellipse
in the plane x = y or x = −y, running from corner to corner of the square at z = 10 up to
z = 16. The groins cross at the centre, (0, 0, 16), where the two vault surfaces are tangent to
each other. The common part's sides are the four 24 × 10 rectangles,
where each block's flat side lies in the same plane as the other's arched one."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.Geom import Geom_BSplineSurface
from OCP.collections import Array1_double, Array1_int, Array2_double, Array2_gp_Pnt
from OCP.gp import gp, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

HALF = 12                       # the square is −12 ≤ x, y ≤ 12
ARCH = [(-12, 10), (0, 26), (12, 10)]  # (y, z) of each control point along y
WEIGHTS = [1, 3 / 5, 1]


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def vault():
    """The first block: its weighted top swept down past the floor, cut at the floor."""
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
    first = vault()
    quarter = gp_Trsf()
    quarter.SetRotation(gp.OZ_s(), math.pi / 2)
    second = BRepBuilderAPI_Transform(first, quarter, True).Shape()
    groin = BRepAlgoAPI_Common(first, second)
    if not groin.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two vaults")
    return groin.Shape()
