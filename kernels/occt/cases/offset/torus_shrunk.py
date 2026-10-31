"""A torus shrunk by 1.5. A solid torus about the z axis, centred on the origin, whose tube has
radius 3 about a middle circle of radius 10, shrunk by 1.5."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from offset import offset

MAJOR_RADIUS = 10
MINOR_RADIUS = 3
SHRINKAGE = 1.5


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
    return offset(torus, -SHRINKAGE)
