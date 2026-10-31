"""A cone grown by 1. A cone on the z axis with a base of radius 3 at z = 0 and its apex at
(0, 0, 4), grown by 1."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from offset import offset

BASE_RADIUS = 3
HEIGHT = 4
GROWTH = 1


def build():
    return offset(BRepPrimAPI_MakeCone(BASE_RADIUS, 0, HEIGHT).Shape(), GROWTH)
