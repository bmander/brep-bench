"""A closed spherical sheet thickened into a hollow ball. The sphere of radius 5 about the origin
as a surface alone, thickened by 1 outward."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from thicken import thicken

RADIUS = 5
THICKNESS = 1


def build():
    # the ball's shell alone, its one face's normal pointing outward
    skin = BRepPrimAPI_MakeSphere(RADIUS).Shell()
    return thicken(skin, THICKNESS)
