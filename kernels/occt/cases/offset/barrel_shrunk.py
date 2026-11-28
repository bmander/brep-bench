"""A ball cut by two planes shrunk by 1. A barrel: the ball of radius 5 centred on the origin cut by
the planes z = −3 and z = 3, keeping the part between them, whose flat ends are disks of radius 4,
shrunk by 1: every point of it at least 1 from its boundary."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from offset import offset

RADIUS = 5
HALF_HEIGHT = 3
SHRINKAGE = 1


def build():
    # the ball's zone between the two latitudes the planes cut it at
    latitude = math.asin(HALF_HEIGHT / RADIUS)
    barrel = BRepPrimAPI_MakeSphere(RADIUS, -latitude, latitude).Shape()
    return offset(barrel, -SHRINKAGE)
