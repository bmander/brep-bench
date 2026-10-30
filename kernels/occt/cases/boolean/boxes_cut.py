"""One block cut by another. Two blocks 10 high, both standing on the xy plane. The first spans
x and y from 0 to 10; the second spans x and y from 5 to 15. Cut the second away from the
first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

HEIGHT = 10


def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, HEIGHT)).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(5, 5, 0), gp_Pnt(15, 15, HEIGHT)).Shape()
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two blocks")
    return cut.Shape()
