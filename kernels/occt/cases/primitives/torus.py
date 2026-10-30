"""A torus: a solid torus centred on the origin, about the z axis. The circle through the middle
of its tube has radius 10, and the tube has radius 2."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

MAJOR_RADIUS = 10  # the circle through the middle of the tube
MINOR_RADIUS = 2   # the tube


def build():
    # gp.XOY: the frame at the origin whose main direction is +z.
    return BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
