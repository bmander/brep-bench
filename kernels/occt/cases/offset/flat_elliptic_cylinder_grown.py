"""A flat elliptic cylinder grown by 1. The ellipse x²/100 + y²/4 ≤ 1 (semi-axes 10 along x and 2
along y) in the plane z = 0 extruded straight up to z = 10, grown by 1: every point within 1 of
it. The ellipse curves at the ends of its long axis more tightly (radius 0.4) than the distance
grown."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp, gp_Elips, gp_Vec

from offset import offset

A, B = 10, 2   # the semi-axes along x and y
HEIGHT = 10
GROWTH = 1


def build():
    # gp.XOY: the frame at the origin, normal +z, its x (the major axis) along +x
    rim = gp_Elips(gp.XOY_s(), A, B)
    end = BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(rim).Edge()).Wire()).Face()
    cylinder = BRepPrimAPI_MakePrism(end, gp_Vec(0, 0, HEIGHT)).Shape()
    return offset(cylinder, GROWTH)
