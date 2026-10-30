"""Two overlapping balls, united. Two balls of radius 5, one centred on the origin and one at
(6, 0, 0). Make their union."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 5


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(6, 0, 0), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return fuse.Shape()
