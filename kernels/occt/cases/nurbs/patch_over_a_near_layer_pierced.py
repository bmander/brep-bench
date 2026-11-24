"""A block under a patch whose folded-back layer lies just beneath it, pierced by a square hole.

The patch is the Bézier surface of degree 4 in u and 1 in v whose control points are
(xᵢ, j, zᵢⱼ), i = 0 … 4, j = 0, 1, with x = 1, 0, −1/3, 0, 1 and z = 799/1000, 401/2000, 4/5,
399/2000, 801/1000 (plus j/10): S(u, v) = (s², v, 1/2 + 3s⁴/10 + s³/1000 + v/10), s = 2u − 1. The
block A stands on z = 0 under the part 13/20 ≤ u ≤ 1 (9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1), its top face on the whole patch,
trimmed. The result is A less the box 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5, −1 ≤ z ≤ 3."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused
from patches import under

XS = (1, 0, -1 / 3, 0, 1)
ZS = (799 / 1000, 401 / 2000, 4 / 5, 399 / 2000, 801 / 1000)  # at v = 0; v = 1 adds a tenth


def build():
    poles = [[(x, j, z + j / 10) for j in (0, 1)] for x, z in zip(XS, ZS)]
    block = under(poles, (13 / 20, 1), (0, 1))
    hole = BRepPrimAPI_MakeBox(gp_Pnt(2 / 5, 3 / 10, -1), gp_Pnt(7 / 10, 3 / 5, 3)).Shape()
    cut = BRepAlgoAPI_Cut(block, hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
