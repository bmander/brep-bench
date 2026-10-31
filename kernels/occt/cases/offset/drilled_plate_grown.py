"""A drilled plate grown by 1. The plate from (−10, −10, 0) to (10, 10, 4), with a hole of radius 3
through it along the z axis, grown by 1."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from offset import offset

HALF_SIDE = 10
THICKNESS = 4
HOLE_RADIUS = 3
GROWTH = 1


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    # The drill runs 1 past both faces of the plate.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), HOLE_RADIUS,
                                     THICKNESS + 2).Shape()
    cut = BRepAlgoAPI_Cut(plate, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return offset(cut.Shape(), GROWTH)
