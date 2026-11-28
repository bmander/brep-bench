"""An L-shaped plate with its inside corner rounded, grown past the round's radius. The L-shaped
plate whose outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5), (5, 5),
(5, 15) and (0, 15), extruded 8 along +z, its concave edge along (5, 5) rounded to radius 4, then
grown by 5: the round closes up inside the grown plate's section."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where, rounded
from offset import offset

OUTLINE = [(0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15)]
THICKNESS = 8
INSIDE_CORNER = (5, 5)
ROUND = 4
GROWTH = 5


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in OUTLINE:
        polygon.Add(gp_Pnt(x, y, 0))
    polygon.Close()
    outline = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    plate = BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
    x, y = INSIDE_CORNER
    corner = edges_where(plate, lambda p: abs(p.X() - x) < 1e-9 and abs(p.Y() - y) < 1e-9)
    # BRepOffsetAPI_MakeOffsetShape reports itself done but makes no shape (a null one), as for
    # offset/drilled-plate-grown-past-its-hole, where a face it moves vanishes; `offset` refuses it.
    return offset(rounded(plate, ROUND, corner, "the inside corner"), GROWTH)
