"""A block under a patch pinched to a point beside it, sliced by a plane passing near the pinch.

The patch is the bicubic Bézier surface whose control points are (xᵢ, xⱼ, 1/2 + sᵢsⱼ), i, j = 0 … 3,
with x = −1, 1, −1, 1 and s = −1, −1/3, 1/3, 1: S(u, v) = (s³, t³, 1/2 + st), s = 2u − 1, t = 2v − 1,
the graph of z = 1/2 + ∛(xy), its parametrization singular at u = v = 1/2 alone. The block A stands
on z = 0 under the part 11/20 ≤ u, v ≤ 1 (1/1000 ≤ x, y ≤ 1), its top face on the whole patch,
trimmed. The result is the part of A where −x + y + z/10 ≤ 13/250."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import Refused
from patches import beyond, under

XS = (-1, 1, -1, 1)
SS = (-1, -1 / 3, 1 / 3, 1)


def build():
    poles = [[(xi, xj, 1 / 2 + si * sj) for xj, sj in zip(XS, SS)] for xi, si in zip(XS, SS)]
    block = under(poles, (11 / 20, 1), (11 / 20, 1))
    # what lies past the plane −x + y + z/10 = 13/250, through (0, 13/250, 0), along (1, 1, 0)
    past = beyond((0.5, 0.5 + 13 / 250, 0), (-1, 1, 0.1), (1, 1, 0))
    cut = BRepAlgoAPI_Cut(block, past)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slice")
    return cut.Shape()
