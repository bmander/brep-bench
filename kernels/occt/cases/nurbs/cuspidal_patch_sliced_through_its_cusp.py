"""A block under a cuspidal patch, sliced by a plane through the cusp beside it.

The patch is the Bézier surface of degree 3 in u and 1 in v whose control points are
(xᵢ, j, zᵢⱼ), i = 0 … 3, j = 0, 1, with x = 1, −1/3, −1/3, 1 and z = −1/2, 3/2, −1/2, 3/2 (plus
j/10): S(u, v) = (s², v, 1/2 + s³ + v/10), s = 2u − 1, its two layers meeting in a cusp along
u = 1/2. The block A stands on z = 0 under the part 11/20 ≤ u ≤ 1 (1/100 ≤ x ≤ 1, 0 ≤ y ≤ 1), its top
face on the whole patch, trimmed. The result is the part of A on or below the plane z = 11y/10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import Refused
from patches import beyond, under

XS = (1, -1 / 3, -1 / 3, 1)
ZS = (-1 / 2, 3 / 2, -1 / 2, 3 / 2)  # at v = 0; v = 1 adds a tenth


def build():
    poles = [[(x, j, z + j / 10) for j in (0, 1)] for x, z in zip(XS, ZS)]
    block = under(poles, (11 / 20, 1), (0, 1))
    # what lies above the plane z = 11y/10: its upward normal (0, −11/10, 1), level along x
    above = beyond((0.5, 0.5, 0.55), (0, -1.1, 1), (1, 0, 0))
    cut = BRepAlgoAPI_Cut(block, above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slice")
    return cut.Shape()
