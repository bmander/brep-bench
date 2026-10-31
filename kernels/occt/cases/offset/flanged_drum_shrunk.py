"""A flanged drum shrunk until its flange is gone. A drum, the cylinder of radius 8 on the z axis
from z = 0 to z = 6, with a thin flange round its side, a ring from radius 8 out to radius 11 and
from z = 2.2 to z = 3.8, shrunk by 1."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from offset import offset

RADIUS = 8
HEIGHT = 6
FLANGE_BOTTOM, FLANGE_TOP = 2.2, 3.8
FLANGE_RADIUS = 11
SHRINKAGE = 1


def build():
    drum = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    flange = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, FLANGE_BOTTOM), gp.DZ_s()),
                                      FLANGE_RADIUS, FLANGE_TOP - FLANGE_BOTTOM).Shape()
    fuse = BRepAlgoAPI_Fuse(drum, flange)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the flange")
    return offset(fuse.Shape(), -SHRINKAGE)
