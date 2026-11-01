"""A square sheet with two holes, thickened to both sides. The square in the xy plane from
(−10, −10, 0) to (10, 10, 0), with two round holes of radius 3 centred on (−5, 0, 0) and
(5, 0, 0), thickened by 1 to each side.

OCCT thickens a sheet to one side only, so the sheet is first moved 1 to its −z side (an offset of
the face alone, BRepOffsetAPI_MakeOffsetShape's simple offset) and that thickened by 2 to +z."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused
from thicken import thicken

HALF_SIDE = 10
HOLES = [(-5, 0), (5, 0)]
HOLE_RADIUS = 3
EACH_SIDE = 1


def build():
    face = BRepBuilderAPI_MakeFace(gp_Pln(), -HALF_SIDE, HALF_SIDE, -HALF_SIDE, HALF_SIDE)
    for x, y in HOLES:
        circle = gp_Circ(gp_Ax2(gp_Pnt(x, y, 0), gp_Dir(0, 0, 1)), HOLE_RADIUS)
        hole = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
        # a hole's wire runs against the outer one, clockwise seen from +z
        face.Add(TopoDS.Wire(hole.Reversed()))
    lower = BRepOffsetAPI_MakeOffsetShape()
    lower.PerformBySimple(face.Face(), -EACH_SIDE)
    if not lower.IsDone():
        raise Refused("BRepOffsetAPI_MakeOffsetShape declined to move the sheet")
    return thicken(lower.Shape(), 2 * EACH_SIDE)
