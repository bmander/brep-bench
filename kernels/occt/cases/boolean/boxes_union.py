"""Two overlapping blocks, united. Two blocks 10 high, both standing on the xy plane. The first
spans x and y from 0 to 10; the second spans x and y from 5 to 15. Make their union. Their tops
lie in one plane, and so do their bottoms."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

HEIGHT = 10


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, HEIGHT)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(5, 5, 0), gp_Pnt(15, 15, HEIGHT)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two blocks")
    # The fuse leaves each top (and each bottom) split where the blocks crossed; merge the
    # pieces lying in one plane into one face each.
    fuse.SimplifyResult()
    return fuse.Shape()
