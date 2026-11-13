"""An L-shaped block shelled inward open at its step, the wall rising past the opening kept. The
outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
shelled inward to a wall 1 thick, open at the face y = 5, the corners sharp."""

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Pnt, gp_Vec

from shell import face_through, shell

OUTLINE = [(0, 0), (20, 0), (20, 20), (15, 20), (15, 5), (0, 5)]
HEIGHT = 20
THICKNESS = 1


def block(outline, height):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in outline:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def build():
    solid = block(OUTLINE, HEIGHT)
    # joined by intersection: the moved faces extended until they meet
    return shell(solid, [face_through(solid, (7.5, 5, 10))], THICKNESS,
                 join=GeomAbs_Intersection)
