"""An L-shaped plate shrunk. The L-shaped plate whose outline in the xy plane is the polygon through
(0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8 along +z."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTLINE = [(0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15)]
THICKNESS = 8
DISTANCE = -1


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    outline = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    plate = BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
    return offset(plate, DISTANCE)
