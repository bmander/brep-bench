"""A plate with a shallow round recess grown by 4, past the recess's radius. The plate from
(−10, −10, 0) to (10, 10, 4), with a round recess of radius 3 sunk 1 into its top about the z axis,
grown by 4: the recess fills, and over it the grown top dips."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from offset import less, offset

HALF_SIDE = 10
THICKNESS = 4
RECESS_RADIUS = 3
RECESS_DEPTH = 1
GROWTH = 4


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    # The cutter runs from the recess's floor to 1 above the plate's top.
    floor = THICKNESS - RECESS_DEPTH
    cutter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, floor), gp.DZ_s()), RECESS_RADIUS,
                                      RECESS_DEPTH + 1).Shape()
    return offset(less(plate, cutter, "the recess"), GROWTH)
