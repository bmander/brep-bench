"""Two cubes joined face to face. Two cubes of side 10, side by side: the first between the
origin and (10, 10, 10), the second between (10, 0, 0) and (20, 10, 10). Join them into one
solid. They touch along a whole face, which must disappear from the result."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(10, 0, 0), gp_Pnt(20, 10, 10)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cubes")
    # The shared face is gone from the fuse; merge the coplanar sides it leaves into single
    # faces, so the result is one 20 × 10 × 10 block of six faces.
    fuse.SimplifyResult()
    return fuse.Shape()
