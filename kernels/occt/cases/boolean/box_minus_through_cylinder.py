"""A cube drilled through. Take a cube of side 10 between the origin and (10, 10, 10). Drill a
hole of radius 3 straight through it along z, centred at x = 5, y = 5. The cutting cylinder runs
past both faces: from z = −1 to z = 11."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

SIDE = 10
HOLE_RADIUS = 3
HOLE_FROM, HOLE_TO = -1, 11  # z, past both faces


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    axis = gp_Ax2(gp_Pnt(5, 5, HOLE_FROM), gp.DZ_s())
    drill = BRepPrimAPI_MakeCylinder(axis, HOLE_RADIUS, HOLE_TO - HOLE_FROM).Shape()
    cut = BRepAlgoAPI_Cut(cube, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
