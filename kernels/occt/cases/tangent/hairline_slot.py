"""A cube with a hairline saw cut. A cube of side 10 between the origin and (10, 10, 10), with a
slot a millionth wide down into it from the top: the block between x = 5 and x = 5.000001,
running right through in y, from z = 5 up through the top face. If you cut it with a block, make
that block run past the cube's front, back and top."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 10
SLOT_X = 5
SLOT_WIDTH = 1e-6
SLOT_FLOOR = 5


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    saw = BRepPrimAPI_MakeBox(gp_Pnt(SLOT_X, -1, SLOT_FLOOR),
                              gp_Pnt(SLOT_X + SLOT_WIDTH, SIDE + 1, SIDE + 1)).Shape()
    cut = BRepAlgoAPI_Cut(cube, saw)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the saw cut")
    return cut.Shape()
