"""A cylindrical sheet thickened into a tube. The whole cylinder of radius 5 about the z axis from
z = 0 to z = 10, one face closed around on itself, thickened by 1 away from the axis."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Cylinder

from thicken import thicken

RADIUS = 5
HEIGHT = 10
THICKNESS = 1


def build():
    # u over its whole period: one face, its seam at u = 0
    cylinder = gp_Cylinder(gp_Ax3(), RADIUS)
    sheet = BRepBuilderAPI_MakeFace(cylinder, 0, 2 * pi, 0, HEIGHT).Face()
    return thicken(sheet, THICKNESS)
