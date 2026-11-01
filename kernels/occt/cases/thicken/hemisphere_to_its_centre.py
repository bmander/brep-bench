"""A hemispherical sheet thickened to its centre. The upper half of the sphere of radius 5 about
the origin, thickened by 5 toward the sphere's centre."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Sphere

from thicken import thicken

RADIUS = 5


def build():
    # the normal points away from the centre, so toward it is a negative distance: the whole radius
    sheet = BRepBuilderAPI_MakeFace(gp_Sphere(gp_Ax3(), RADIUS), 0, 2 * pi, 0, pi / 2).Face()
    return thicken(sheet, -RADIUS)
