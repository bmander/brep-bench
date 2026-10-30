"""A plate drilled nine times. A plate from the origin to (100, 20, 10). Drill nine holes of
radius 2 through it along z, at y = 10 and x = 10, 20, 30, … 90. Drill them one after another,
each hole its own cut."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

LENGTH, WIDTH, THICKNESS = 100, 20, 10
HOLE_RADIUS = 2
HOLE_Y = 10
HOLE_XS = range(10, 100, 10)  # 10, 20, … 90


def build():
    plate = BRepPrimAPI_MakeBox(LENGTH, WIDTH, THICKNESS).Shape()
    for x in HOLE_XS:
        # Each drill runs 1 past both faces of the plate.
        axis = gp_Ax2(gp_Pnt(x, HOLE_Y, -1), gp.DZ_s())
        drill = BRepPrimAPI_MakeCylinder(axis, HOLE_RADIUS, THICKNESS + 2).Shape()
        cut = BRepAlgoAPI_Cut(plate, drill)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined the hole at x = {x}")
        plate = cut.Shape()
    return plate
