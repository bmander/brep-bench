"""A torus hollowed, its cavity closed. A solid torus about the z axis, centred on the origin, whose
tube has radius 2 about a middle circle of radius 6. Hollow it to a wall 0.5 thick with no opening:
the torus stays as the outside, and moved 0.5 inward it is the inside, the torus whose tube has
radius 1.5 about the same middle circle. The result is the torus less that torus, one solid bounded
by two tori, the inner one facing in, towards the void, which goes round the hole."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from shell import hollow

MAJOR, MINOR = 6, 2
THICKNESS = 0.5


def build():
    # MakeThickSolid wants an opening; with none, the torus's shell and its shrunk copy's, reversed
    return hollow(BRepPrimAPI_MakeTorus(MAJOR, MINOR).Shape(), THICKNESS)
