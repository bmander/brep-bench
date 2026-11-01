"""A half-pipe sheet thickened toward its axis. The half of the cylinder of radius 5 about the
z axis where y ≥ 0, from z = 0 to z = 10, thickened by 2 toward the axis."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Cylinder

from thicken import thicken

RADIUS = 5
HEIGHT = 10
THICKNESS = 2


def build():
    # the cylinder's normal points away from the axis, so toward it is a negative distance
    cylinder = gp_Cylinder(gp_Ax3(), RADIUS)
    sheet = BRepBuilderAPI_MakeFace(cylinder, 0, pi, 0, HEIGHT).Face()
    return thicken(sheet, -THICKNESS)
