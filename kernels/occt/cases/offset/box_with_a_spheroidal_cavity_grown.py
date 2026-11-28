"""A box with a spheroidal cavity grown by 1. The cube from (−10, −10, −10) to (10, 10, 10)
with a closed cavity at its middle, the oblate spheroid (x² + y²)/36 + z²/9 ≤ 1
(geometry/oblate-spheroid), taken out of it. Grown by 1, the cube grows as a box does and the
cavity shrinks to the spheroid's inner parallel body at distance 1.
BRepOffsetAPI_MakeOffsetShape declines a solid with a cavity, a second shell, whatever its join."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import other_case
from offset import less, offset

HALF_SIDE = 10
GROWTH = 1


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, -HALF_SIDE),
                               gp_Pnt(HALF_SIDE, HALF_SIDE, HALF_SIDE)).Shape()
    cavity = other_case("geometry/oblate-spheroid").build()
    return offset(less(cube, cavity, "the spheroidal cavity"), GROWTH)
