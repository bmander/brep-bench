"""A sheet creased to 60°, thickened outside its crease, the corner kept sharp. The floor from
(0, 0, 0) to (10, 8, 0) and the wall through (0, 0, 0), (0, 8, 0), (5, 8, 5√3) and (5, 0, 5√3),
leaning over it at 60°, thickened by 1 below the floor and above the wall, the moved faces
extended until they meet."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Pnt, gp_Vec

from thicken import thicken

LEG = 10
WIDTH = 8
THICKNESS = 1
CREASE = 60  # degrees between the floor and the wall


def sheet():
    """The floor and the wall: the polyline (10, 0, 0), (0, 0, 0), (5, 0, 5√3) swept along +y."""
    angle = math.radians(CREASE)
    crease = BRepBuilderAPI_MakePolygon(
        gp_Pnt(LEG, 0, 0), gp_Pnt(0, 0, 0),
        gp_Pnt(LEG * math.cos(angle), 0, LEG * math.sin(angle))).Wire()
    return BRepPrimAPI_MakePrism(crease, gp_Vec(0, WIDTH, 0)).Shape()


def build():
    # swept along +y, the floor's normal is +z and the wall's points over the floor: to the
    # inside of the crease, so the sheet is thickened against its normals
    return thicken(sheet(), -THICKNESS, GeomAbs_Intersection)
