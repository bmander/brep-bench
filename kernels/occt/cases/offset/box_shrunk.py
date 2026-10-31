"""A box shrunk by 1. The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1
from its outside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from offset import offset

SIZE = (10, 8, 6)
SHRINKAGE = 1


def build():
    return offset(BRepPrimAPI_MakeBox(*SIZE).Shape(), -SHRINKAGE)
