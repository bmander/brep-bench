"""A U-shaped plate grown by 1. Its outline in the xy plane is the polygon through (0, 0), (9, 0),
(9, 9), (6, 9), (6, 3), (3, 3), (3, 9) and (0, 9), a 9 × 9 square with a slot 3 wide cut 6 deep
into it, extruded 2 along +z; grown by 1, the slot's two inside corners stay sharp."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTLINE = [(0, 0), (9, 0), (9, 9), (6, 9), (6, 3), (3, 3), (3, 9), (0, 9)]
THICKNESS = 2
GROWTH = 1


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    outline = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    plate = BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
    return offset(plate, GROWTH)
