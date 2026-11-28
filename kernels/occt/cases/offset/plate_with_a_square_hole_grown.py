"""A plate with a square hole grown by 1. The plate from (0, 0, 0) to (10, 10, 2) with a square hole
through it, the square from (3, 3) to (7, 7), grown by 1: every point within 1 of it. The hole's
four vertical edges are concave and stay sharp."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from offset import offset

OUTSIDE = [(0, 0), (10, 0), (10, 10), (0, 10)]
HOLE = [(3, 3), (3, 7), (7, 7), (7, 3)]   # clockwise, as a hole in the face
THICKNESS = 2
GROWTH = 1


def loop(corners):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in corners:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    return polygon.Wire()


def build():
    # the plate's section, the outer square with the hole's square as a second wire, extruded
    section = BRepBuilderAPI_MakeFace(loop(OUTSIDE))
    section.Add(loop(HOLE))
    plate = BRepPrimAPI_MakePrism(section.Face(), gp_Vec(0, 0, THICKNESS)).Shape()
    return offset(plate, GROWTH)
