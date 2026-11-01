"""A box hollowed, its cavity closed. The box from the origin to (10, 8, 6), hollowed to a wall 1 thick
with no opening: every face stays as the outside, and each is moved 1 inward along its normal to
make the inside. The cavity is closed: the result is the box less the box from (1, 1, 1) to (9, 7,
5), one solid bounded by two shells, the outer one facing out and the cavity's facing in, towards
the void."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from shell import hollow

SIZE = (10, 8, 6)
THICKNESS = 1


def build():
    # MakeThickSolid wants an opening; with none, the box's shell and its shrunk copy's, reversed
    return hollow(BRepPrimAPI_MakeBox(*SIZE).Shape(), THICKNESS)
