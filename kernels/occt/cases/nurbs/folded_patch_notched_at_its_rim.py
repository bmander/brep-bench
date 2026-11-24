"""A block under a patch folded over its own rim, notched across the rim.

The patch is the Bézier surface of degree 4 in u and 1 in v whose control points are
(xᵢ, j, zᵢⱼ), i = 0 … 4, j = 0, 1, with x = 1, 0, −1/3, 0, 1 and z = 4/5, 1/5, 4/5, 1/5, 4/5 (plus
j/10): S(u, v) = (s², v, 1/2 + 3s⁴/10 + v/10), s = 2u − 1, folding back over itself along u = 1/2.
The block A stands on z = 0 under the part 13/20 ≤ u ≤ 1 (9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1), its top face on
the whole patch, trimmed. The result is A less the box 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused
from patches import under

XS = (1, 0, -1 / 3, 0, 1)
ZS = (4 / 5, 1 / 5, 4 / 5, 1 / 5, 4 / 5)  # at v = 0; v = 1 adds a tenth


def build():
    poles = [[(x, j, z + j / 10) for j in (0, 1)] for x, z in zip(XS, ZS)]
    block = under(poles, (13 / 20, 1), (0, 1))
    notch = BRepPrimAPI_MakeBox(gp_Pnt(1 / 20, -1, -1), gp_Pnt(1 / 2, 1 / 2, 3)).Shape()
    cut = BRepAlgoAPI_Cut(block, notch)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the notch")
    return cut.Shape()
