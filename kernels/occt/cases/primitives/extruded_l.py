"""An L-shaped plate. Its outline in the xy plane is the polygon through (0, 0), (20, 0),
(20, 5), (5, 5), (5, 15) and (0, 15). Extrude it 8 along +z."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

OUTLINE = [(0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15)]
THICKNESS = 8


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    outline = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
