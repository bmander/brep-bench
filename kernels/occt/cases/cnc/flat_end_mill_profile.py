"""A groove profiled round a cross by a flat end mill in three step-downs. The plate is the box
from (0, 0, −15) to (120, 90, 0); a flat end mill of radius 3, 20 long from its tip, upright,
plunges at (60, 10) and runs anticlockwise round the outline of a cross (a bar 80 by 30 across a
bar 30 by 70, every corner rounded to 6) at z = −3, −6 and −9, then rises. The solid is the plate
less everything the mill passes through."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from cnc import Program, cut_all, flat, rounded_loop

CROSS = [(45, 10), (75, 10), (75, 30), (100, 30), (100, 60), (75, 60), (75, 80), (45, 80),
         (45, 60), (20, 60), (20, 30), (45, 30)]
START = (60, 10, 5)


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -15), gp_Pnt(120, 90, 0)).Shape()
    p = Program(flat(3, 20), START)
    for z in (-3, -6, -9):
        p.line((START[0], START[1], z))
        rounded_loop(p, CROSS, 6, z)
    p.line(START)
    return cut_all(plate, p.swept)
