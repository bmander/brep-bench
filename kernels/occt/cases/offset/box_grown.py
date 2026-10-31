"""A box grown by 1. The box from the origin to (10, 8, 6), grown by 1: every point within 1 of
it."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from offset import offset

SIZE = (10, 8, 6)
GROWTH = 1


def build():
    return offset(BRepPrimAPI_MakeBox(*SIZE).Shape(), GROWTH)
