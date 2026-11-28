"""A plate with a wide hole grown by 20, exactly the hole's radius. The plate from (−30, −30, 0) to
(30, 30, 4), with a hole of radius 20 through it along the z axis, grown by 20: the hole fills, and
each end of it leaves a dimple, part of a horn torus, coming to a point on the axis."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from offset import less, offset

HALF_SIDE = 30
THICKNESS = 4
HOLE_RADIUS = 20
GROWTH = 20


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    # The drill runs 1 past both faces of the plate.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), HOLE_RADIUS,
                                     THICKNESS + 2).Shape()
    return offset(less(plate, drill, "the hole"), GROWTH)
