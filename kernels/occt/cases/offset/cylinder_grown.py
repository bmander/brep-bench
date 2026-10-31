"""A cylinder grown by 1. A cylinder of radius 5 on the z axis, from z = 0 to z = 10, grown by 1."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from offset import offset

RADIUS = 5
HEIGHT = 10
GROWTH = 1


def build():
    return offset(BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape(), GROWTH)
