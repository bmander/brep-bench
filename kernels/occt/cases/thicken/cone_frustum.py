"""A conical sheet thickened along its normal. The side of the cone frustum about the z axis whose
radius runs from 6 at z = 0 to 3 at z = 4, thickened by 1 along its normal away from the axis."""
from math import atan2, pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Ax3, gp_Cone

from thicken import thicken

BOTTOM_RADIUS = 6
TOP_RADIUS = 3
HEIGHT = 4
THICKNESS = 1


def build():
    # radius 6 at the cone's origin, narrowing as it rises: a negative half angle; v runs along
    # the slant, 5 from rim to rim
    half_angle = -atan2(BOTTOM_RADIUS - TOP_RADIUS, HEIGHT)
    cone = gp_Cone(gp_Ax3(), half_angle, BOTTOM_RADIUS)
    slant = ((BOTTOM_RADIUS - TOP_RADIUS) ** 2 + HEIGHT ** 2) ** 0.5
    sheet = BRepBuilderAPI_MakeFace(cone, 0, 2 * pi, 0, slant).Face()
    return thicken(sheet, THICKNESS)
