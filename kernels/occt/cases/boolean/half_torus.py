"""Half a torus. A torus about the z axis, centred on the origin, whose tube's centre circle has
radius 10 and whose tube has radius 2. Cut away everything where x < 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused

MAJOR_RADIUS = 10
MINOR_RADIUS = 2


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
    # "Everything where x < 0" is the half-space bounded by the plane x = 0 on the side of a
    # point with negative x.
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DX_s())).Face()
    negative_x = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(-1, 0, 0)).Solid()
    cut = BRepAlgoAPI_Cut(torus, negative_x)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
