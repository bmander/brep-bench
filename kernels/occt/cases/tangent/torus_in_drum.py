"""A torus through a drum that touches its outer equator. A solid torus about the z axis, the
middle circle of its tube of radius 10 in the xy plane and the tube of radius 3, united with a
cylinder of radius 13 on the z axis, from z = −1 to z = 1. The torus touches the drum's wall from
inside along its outer equator."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

MAJOR = 10
MINOR = 3
DRUM_RADIUS = 13
DRUM_HALF_HEIGHT = 1


def build():
    torus = BRepPrimAPI_MakeTorus(MAJOR, MINOR).Shape()
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -DRUM_HALF_HEIGHT), gp.DZ_s()),
                                    DRUM_RADIUS, 2 * DRUM_HALF_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(torus, drum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the torus and the drum")
    return fuse.Shape()
