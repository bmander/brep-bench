"""A drilled plate ten times larger, grown by its hole's radius. The plate from (−100, −100, 0) to
(100, 100, 40), with a hole of radius 30 through it along the z axis, grown by 30: the hole fills,
and each end of it leaves a dimple in the grown face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from offset import offset

HALF_SIDE = 100
THICKNESS = 40
HOLE_RADIUS = 30
GROWTH = 30


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    # The drill runs 10 past both faces of the plate.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), HOLE_RADIUS,
                                     THICKNESS + 20).Shape()
    cut = BRepAlgoAPI_Cut(plate, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    # As at a tenth of the size, BRepOffsetAPI_MakeOffsetShape reports itself done here but gives
    # a null shape: it does not close the hole onto its axis.
    return offset(cut.Shape(), GROWTH)
