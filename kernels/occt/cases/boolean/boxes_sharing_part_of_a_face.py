"""A cube on a cube, overhanging. Two cubes of side 10, joined into one solid. The first lies
between the origin and (10, 10, 10). The second lies between (5, 0, 10) and (15, 10, 20): it
stands on the first, and its bottom covers half of the first's top."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    lower = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(5, 0, 10), gp_Pnt(15, 10, 20)).Shape()
    fuse = BRepAlgoAPI_Fuse(lower, upper)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two cubes")
    # The cubes' sides at y = 0 and at y = 10 are coplanar; merge each pair into one face.
    fuse.SimplifyResult()
    return fuse.Shape()
