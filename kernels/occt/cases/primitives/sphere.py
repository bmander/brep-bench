"""A ball: a solid ball of radius 5 centred on the origin."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

CENTRE = gp_Pnt(0, 0, 0)
RADIUS = 5


def build():
    return BRepPrimAPI_MakeSphere(CENTRE, RADIUS).Shape()
