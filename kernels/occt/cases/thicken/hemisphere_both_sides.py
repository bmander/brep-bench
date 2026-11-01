"""A hemispherical sheet thickened to both sides. The upper half of the sphere of radius 5 about
the origin, thickened by 1 to each side.

OCCT thickens a sheet to one side only, so the sheet is first moved 1 toward its centre (an offset
of the face alone, BRepOffsetAPI_MakeOffsetShape's simple offset) and that thickened by 2
outward."""
from math import pi

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape
from OCP.gp import gp_Ax3, gp_Sphere

from bench import Refused
from thicken import thicken

RADIUS = 5
EACH_SIDE = 1


def build():
    # latitude v from 0 (the equator) to π/2 (the pole); the normal points away from the centre
    sheet = BRepBuilderAPI_MakeFace(gp_Sphere(gp_Ax3(), RADIUS), 0, 2 * pi, 0, pi / 2).Face()
    inner = BRepOffsetAPI_MakeOffsetShape()
    inner.PerformBySimple(sheet, -EACH_SIDE)
    if not inner.IsDone():
        raise Refused("BRepOffsetAPI_MakeOffsetShape declined to move the sheet")
    return thicken(inner.Shape(), 2 * EACH_SIDE)
