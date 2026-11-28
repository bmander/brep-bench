"""A five-pointed star prism grown by a half. Its outline in the xy plane is the polygon through ten
points around the origin, alternately 10 and 5 from it, the first at 10 on the +y axis and each
next 36° further anticlockwise, extruded 2 along +z; grown by 1/2, its inside corners stay sharp."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTER, INNER = 10, 5  # how far the points and the inside corners are from the origin
THICKNESS = 2
GROWTH = 0.5


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for i in range(10):
        radius = OUTER if i % 2 == 0 else INNER
        turn = math.radians(90 + 36 * i)
        polygon.Add(gp_Pnt(radius * math.cos(turn), radius * math.sin(turn), 0))
    polygon.Close()
    outline = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    star = BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
    return offset(star, GROWTH)
