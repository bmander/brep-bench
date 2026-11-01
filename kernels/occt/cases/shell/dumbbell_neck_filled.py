"""A dumbbell plate shelled thicker than its neck: the inside walls cross and the neck fills. A
dumbbell-shaped plate: its outline in the xy plane is the polygon through (0, 0), (10, 0), (10, 4),
(14, 4), (14, 0), (24, 0), (24, 10), (14, 10), (14, 6), (10, 6), (10, 10) and (0, 10), two 10 × 10
squares joined by a neck 4 long and 2 wide, extruded 6 along +z. Shell it inward to a wall 1.5
thick, open at its top face, z = 6: the wall is every point of the plate within 1.5 of its faces
other than the top."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from shell import face_through, shell

OUTLINE = [(0, 0), (10, 0), (10, 4), (14, 4), (14, 0), (24, 0), (24, 10), (14, 10), (14, 6),
           (10, 6), (10, 10), (0, 10)]
HEIGHT = 6
THICKNESS = 1.5


def plate(outline, height):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in outline:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def build():
    solid = plate(OUTLINE, HEIGHT)
    # joined by arcs: the pockets' bulges at the neck's mouths are arcs about its corners
    return shell(solid, [face_through(solid, (5, 5, HEIGHT))], THICKNESS)
