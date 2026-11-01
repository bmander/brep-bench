"""A ball hollowed, its cavity closed. A ball of radius 5 centred on the origin, hollowed to a wall 1
thick with no opening: its sphere stays as the outside, and moved 1 inward it is the inside, the
sphere of radius 4 about the origin. The result is the ball less the ball of radius 4, one solid
bounded by two spheres, the inner one facing in, towards the void."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from shell import hollow

RADIUS = 5
THICKNESS = 1


def build():
    # MakeThickSolid wants an opening; with none, the ball's shell and its shrunk copy's, reversed
    return hollow(BRepPrimAPI_MakeSphere(RADIUS).Shape(), THICKNESS)
