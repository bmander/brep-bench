"""A creased sheet thickened outside its crease, the corner rounded. The floor from (0, 0, 0) to
(10, 8, 0) and the wall from (0, 0, 0) to (0, 8, 10), thickened by 1 to −z and −x, the gap at the
crease closed by a quarter cylinder."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Arc
from OCP.gp import gp_Pnt, gp_Vec

from thicken import thicken

LEG = 10
WIDTH = 8
THICKNESS = 1


def sheet():
    """The floor and the wall: the polyline (10, 0, 0), (0, 0, 0), (0, 0, 10) swept along +y."""
    crease = BRepBuilderAPI_MakePolygon(gp_Pnt(LEG, 0, 0), gp_Pnt(0, 0, 0),
                                        gp_Pnt(0, 0, LEG)).Wire()
    return BRepPrimAPI_MakePrism(crease, gp_Vec(0, WIDTH, 0)).Shape()


def build():
    # swept along +y, the floor's normal is +z and the wall's +x: to the inside of the crease
    return thicken(sheet(), -THICKNESS, GeomAbs_Arc)
