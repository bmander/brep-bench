"""A grooved drum grown until its groove closes. A drum, the cylinder of radius 10 on the z axis
from z = 0 to z = 10, with a groove round its side from z = 4.2 to z = 5.8 cut 3 deep (to radius
7), grown by 1."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from offset import offset

RADIUS = 10
HEIGHT = 10
GROOVE_BOTTOM, GROOVE_TOP = 4.2, 5.8
GROOVE_RADIUS = 7
GROWTH = 1


def build():
    drum = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    # The groove is a ring: a disc reaching past the drum's side, less the core it leaves.
    axis = gp_Ax2(gp_Pnt(0, 0, GROOVE_BOTTOM), gp.DZ_s())
    ring = BRepAlgoAPI_Cut(
        BRepPrimAPI_MakeCylinder(axis, RADIUS + 1, GROOVE_TOP - GROOVE_BOTTOM).Shape(),
        BRepPrimAPI_MakeCylinder(axis, GROOVE_RADIUS, GROOVE_TOP - GROOVE_BOTTOM).Shape())
    cut = BRepAlgoAPI_Cut(drum, ring.Shape())
    if not ring.IsDone() or not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return offset(cut.Shape(), GROWTH)
