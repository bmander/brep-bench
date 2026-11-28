"""A plus-shaped plate grown by 1. The plate whose outline in the xy plane is the polygon through
(3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3),
extruded 2 along +z, grown by 1: every point within 1 of it. Its four concave vertical edges stay
sharp."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTLINE = [(3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3),
           (3, 3)]
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
