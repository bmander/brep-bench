"""An L-shaped plate shelled with its top open, the wall at its inside corner rounded. An L-shaped
plate: its outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15)
and (0, 15), extruded 8 along +z. Its one concave edge is the vertical line x = 5, y = 5. Shell it
inward to a wall 1 thick, open at its top face, z = 8, the wall being every point of the plate
within 1 of its faces other than the top."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from shell import face_through, shell

OUTLINE = [(0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15)]
HEIGHT = 8
THICKNESS = 1


def plate(outline, height):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in outline:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def build():
    solid = plate(OUTLINE, HEIGHT)
    # joined by arcs: the wall round the concave edge is a quarter cylinder about it
    return shell(solid, [face_through(solid, (2, 2, HEIGHT))], THICKNESS)
