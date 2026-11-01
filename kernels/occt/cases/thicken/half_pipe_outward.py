"""A half-pipe sheet thickened away from its axis. The half of the cylinder of radius 5 about the
z axis where y ≥ 0, from z = 0 to z = 10, thickened by 1 away from the axis."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Cylinder

from thicken import thicken

RADIUS = 5
HEIGHT = 10
THICKNESS = 1


def build():
    # the cylinder's angle u from 0 to π is the half where y ≥ 0; its normal points away from the
    # axis, so a positive distance thickens it outward
    cylinder = gp_Cylinder(gp_Ax3(), RADIUS)
    sheet = BRepBuilderAPI_MakeFace(cylinder, 0, pi, 0, HEIGHT).Face()
    return thicken(sheet, THICKNESS)
