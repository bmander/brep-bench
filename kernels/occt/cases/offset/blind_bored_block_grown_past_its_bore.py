"""A block with a deep blind bore grown by 4, past the bore's radius. The block from (−10, −10, 0)
to (10, 10, 10) with a blind bore of radius 3 down its z axis from the top, 8 deep, to a flat
floor at z = 2, grown by 4: the bore fills, and its mouth leaves a dimple in the grown top."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from offset import less, offset

HALF_SIDE = 10
HEIGHT = 10
BORE_RADIUS = 3
FLOOR = 2
GROWTH = 4


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, HEIGHT)).Shape()
    # The drill runs from the floor to 1 past the top.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, FLOOR), gp.DZ_s()), BORE_RADIUS,
                                     HEIGHT - FLOOR + 1).Shape()
    # BRepOffsetAPI_MakeOffsetShape reports itself done but makes no shape (a null one), as for
    # offset/drilled-plate-grown-past-its-hole, where a face it moves vanishes; `offset` refuses it.
    return offset(less(block, drill, "the bore"), GROWTH)
