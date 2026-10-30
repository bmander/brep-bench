"""A pin lug. A flat lug: a plate from (0, −5, 0) to (80, 5, 40), so it is 80 long in x, 40 tall
in z and 10 thick in y. A round pin hole of radius 8 goes straight through its thickness, along y,
its axis the line x = 60, z = 20. If you cut the hole with a cylinder, make it run past both faces
of the plate."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

CORNER = (0, -5, 0)
OPPOSITE = (80, 5, 40)
HOLE_RADIUS = 8
HOLE_X, HOLE_Z = 60, 20


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(*CORNER), gp_Pnt(*OPPOSITE)).Shape()
    # The drill runs along +y from y = −6 to y = 6, 1 past both faces.
    axis = gp_Ax2(gp_Pnt(HOLE_X, CORNER[1] - 1, HOLE_Z), gp.DY_s())
    drill = BRepPrimAPI_MakeCylinder(axis, HOLE_RADIUS, OPPOSITE[1] - CORNER[1] + 2).Shape()
    cut = BRepAlgoAPI_Cut(plate, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pin hole")
    return cut.Shape()
