"""A quarter of a washer. In the xz plane, take the rectangle with x from 6 to 10 and z from 0
to 4. Revolve it 90° about the z axis, right-handed: from +x towards +y."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

INNER, OUTER = 6, 10  # x
BOTTOM, TOP = 0, 4    # z
ANGLE = 90            # degrees


def build():
    rectangle = BRepBuilderAPI_MakePolygon(
        gp_Pnt(INNER, 0, BOTTOM), gp_Pnt(OUTER, 0, BOTTOM),
        gp_Pnt(OUTER, 0, TOP), gp_Pnt(INNER, 0, TOP), True).Wire()
    section = BRepBuilderAPI_MakeFace(rectangle).Face()
    # gp.OZ is the z axis; a positive angle turns right-handed about it, from +x towards +y.
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s(), math.radians(ANGLE)).Shape()
