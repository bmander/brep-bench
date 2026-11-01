"""An elliptic cylinder. A solid elliptic cylinder standing on the xy plane: the ellipse x²/25 +
y²/9 ≤ 1 (semi-axes 5 along x and 3 along y, centred on the z axis) in the plane z = 0, extruded
straight up to z = 10. It has two flat elliptic ends and one curved side, which is not a circular
cylinder anywhere."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp, gp_Elips, gp_Vec

A, B = 5, 3   # the semi-axes along x and y
HEIGHT = 10


def build():
    # gp.XOY: the frame at the origin, normal +z, its x (the major axis) along +x
    rim = gp_Elips(gp.XOY_s(), A, B)
    end = BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(rim).Edge()).Wire()).Face()
    return BRepPrimAPI_MakePrism(end, gp_Vec(0, 0, HEIGHT)).Shape()
