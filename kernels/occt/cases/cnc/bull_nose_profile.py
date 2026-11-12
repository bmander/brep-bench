"""A groove profiled round a six-lobed flower by a bull-nose mill in three step-downs. The plate is
the box from (0, 0, −16) to (120, 120, 0); a bull-nose mill of radius 4, its corner rounded to
1.5, 25 long from its tip, upright, plunges at (102, 60) and runs anticlockwise round the flower
about (60, 60) — lobes of radius 12 about points 30 from its centre at 0°, 60°, … 300°, valleys
about points 40 from it at 30°, 90°, … 330°, each tangent to the lobes beside it — at z = −4, −8
and −10, then rises. The solid is the plate less everything the mill passes through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from cnc import Program, bull, cut_all

CENTRE = (60, 60)
LOBE = 12
VALLEY = math.sqrt(2500 - 1200 * math.sqrt(3)) - LOBE
START = (102, 60, 5)


def about(r, degrees):
    a = math.radians(degrees)
    return (CENTRE[0] + r * math.cos(a), CENTRE[1] + r * math.sin(a))


def toward(a, b, r, z):
    """The point `r` from `a` toward `b`, at height `z`: where circles about them touch."""
    length = math.hypot(b[0] - a[0], b[1] - a[1])
    return (a[0] + r * (b[0] - a[0]) / length, a[1] + r * (b[1] - a[1]) / length, z)


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -16), gp_Pnt(120, 120, 0)).Shape()
    p = Program(bull(4, 1.5, 25), START)
    for z in (-4, -8, -10):
        p.line((START[0], START[1], z))
        for k in range(6):
            lobe, dip, after = about(30, 60 * k), about(40, 60 * k + 30), about(30, 60 * k + 60)
            p.arc(lobe, toward(lobe, dip, LOBE, z), anticlockwise=True)
            p.arc(dip, toward(dip, after, VALLEY, z), anticlockwise=False)
        p.arc(about(30, 0), (START[0], START[1], z), anticlockwise=True)
    p.line(START)
    return cut_all(plate, p.swept)
