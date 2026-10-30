"""A hemisphere. A ball of radius 5 centred on the origin, with everything below the xy plane
cut away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused

RADIUS = 5


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    # Everything below the xy plane: the half-space on the side of a point with negative z.
    xy_plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DZ_s())).Face()
    below = BRepPrimAPI_MakeHalfSpace(xy_plane, gp_Pnt(0, 0, -1)).Solid()
    cut = BRepAlgoAPI_Cut(ball, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
