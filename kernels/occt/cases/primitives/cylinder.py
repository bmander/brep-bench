"""A cylinder: radius 5 and height 10, its axis the z axis, standing on the xy plane (it runs
from z = 0 to z = 10)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

RADIUS = 5
HEIGHT = 10


def build():
    # With no axis given, OCCT stands the cylinder on the origin along +z.
    return BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
