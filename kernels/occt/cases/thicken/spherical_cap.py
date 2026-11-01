"""A spherical cap thickened outward, its rim a cone. The cap of the sphere of radius 10 about the
origin where z ≥ 6, thickened by 2 away from the sphere's centre."""
from math import asin, pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Sphere

from thicken import thicken

RADIUS = 10
FLOOR = 6
THICKNESS = 2


def build():
    # latitude v from where z = 6 to the pole
    sphere = gp_Sphere(gp_Ax3(), RADIUS)
    sheet = BRepBuilderAPI_MakeFace(sphere, 0, 2 * pi, asin(FLOOR / RADIUS), pi / 2).Face()
    return thicken(sheet, THICKNESS)
