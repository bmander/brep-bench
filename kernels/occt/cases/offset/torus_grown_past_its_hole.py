"""A torus grown past its hole. A solid torus about the z axis, centred on the origin, whose tube
has radius 2 about a middle circle of radius 4, grown by 3."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from offset import offset

MAJOR_RADIUS = 4
MINOR_RADIUS = 2
GROWTH = 3


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
    return offset(torus, GROWTH)
