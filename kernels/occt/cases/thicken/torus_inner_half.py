"""The inner half of a torus thickened toward its axis. The torus about the z axis whose tube has
radius 2 about a middle circle of radius 6, its surface's inner half (no farther than 6 from the
axis), thickened by 1 away from the middle circle."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Torus

from thicken import thicken

MAJOR_RADIUS = 6
MINOR_RADIUS = 2
THICKNESS = 1


def build():
    # v round the tube from π/2 (its top) through π (nearest the axis) to 3π/2 (its bottom); the
    # normal points away from the middle circle
    torus = gp_Torus(gp_Ax3(), MAJOR_RADIUS, MINOR_RADIUS)
    sheet = BRepBuilderAPI_MakeFace(torus, 0, 2 * pi, pi / 2, 3 * pi / 2).Face()
    return thicken(sheet, THICKNESS)
