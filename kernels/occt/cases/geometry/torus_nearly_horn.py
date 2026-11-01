"""A torus whose hole is nearly closed. A solid torus centred on the origin, about the z axis: the
circle through the middle of its tube has radius 5, and the tube has radius 4.99. Its hole, in the
plane z = 0, is a circle of radius 0.01 about the axis, two hundredths across, and it widens away
from that plane only slowly: the surface passes within 0.01 of the axis and curves round it with a
radius of 4.99."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

MAJOR_RADIUS = 5  # the circle through the middle of the tube
MINOR_RADIUS = 4.99  # the tube


def build():
    return BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
