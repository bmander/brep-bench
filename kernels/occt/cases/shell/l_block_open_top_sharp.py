"""An L-shaped plate shelled with its top open, the wall at its inside corner kept sharp. An L-shaped
plate: its outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15)
and (0, 15), extruded 8 along +z. Its one concave edge is the vertical line x = 5, y = 5. Shell it
inward to a wall 1 thick, open at its top face, z = 8. The outside of the wall is the plate's other
faces, unmoved, and the inside is each of them moved 1 inward along its normal, onto the surface
offset 1 from it. The moved faces are extended or trimmed where they meet one another, and run on
until they meet the surface of an opening, which is left as a rim between the outside and the
inside. Here every pair of moved faces is extended until they meet, the concave edge included, so
that no round is added."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Intersection
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
    # joined by intersection: the moved walls at the concave edge extended until they meet
    return shell(solid, [face_through(solid, (2, 2, HEIGHT))], THICKNESS,
                 join=GeomAbs_Intersection)
