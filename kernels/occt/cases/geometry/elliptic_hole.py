"""A block with an elliptic hole. A block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8, with a hole of
elliptic section bored straight through it along the z axis: everything with x²/36 + y²/9 < 1
(semi-axes 6 along x and 3 along y) is removed, from the bottom face to the top. The hole's wall
stays clear of the block's sides, 6 from the ends at x = ±12 and 5 from the sides at y = ±8. If you
cut it with an elliptic cylinder, make it run past both faces."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt, gp_Vec

from bench import Refused

A, B = 6, 3  # the hole's semi-axes along x and y
DEPTH = 8


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-12, -8, 0), gp_Pnt(12, 8, DEPTH)).Shape()
    # the drill: the ellipse at z = −1 extruded to z = DEPTH + 1, past both faces
    rim = gp_Elips(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), A, B)
    disc = BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(rim).Edge()).Wire()).Face()
    drill = BRepPrimAPI_MakePrism(disc, gp_Vec(0, 0, DEPTH + 2)).Shape()
    cut = BRepAlgoAPI_Cut(block, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to bore the elliptic hole")
    return cut.Shape()
