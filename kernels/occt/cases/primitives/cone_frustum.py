"""A truncated cone on the z axis, from z = 0 to z = 6, of radius 5 at its base (z = 0) and
radius 2 at its top (z = 6)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

BASE_RADIUS = 5
TOP_RADIUS = 2
HEIGHT = 6


def build():
    # A cone with two non-zero radii is OCCT's frustum, standing on the origin along +z.
    return BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
