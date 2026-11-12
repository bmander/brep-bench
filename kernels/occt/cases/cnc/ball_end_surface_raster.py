"""A free-form surface finished by a ball-end mill in 41 three-axis raster passes. The plate is the
box from (0, 0, −20) to (100, 80, 0); a ball-end mill of radius 3, 30 long from its tip, upright,
runs its tip straight between the points (x, 2j, f(x, 2j)) of the surface
f(x, y) = −5 + 4 sin(πx/50) sin(πy/40), x = −5, 0, … 105, along y = 2j for j = 0 to 40, zigzag:
the even passes along +x and the odd ones back, each next pass from the last's end. It plunges
from (−5, 0, 5) first and rises to (−5, 80, 5) last. The solid is the plate less everything the
mill passes through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from cnc import Program, ball, cut_all


def surface(x, y):
    return -5 + 4 * math.sin(math.pi * x / 50) * math.sin(math.pi * y / 40)


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, -20), gp_Pnt(100, 80, 0)).Shape()
    p = Program(ball(3, 30), (-5, 0, 5))
    for j in range(41):
        y = 2 * j
        points = range(23) if j % 2 == 0 else reversed(range(23))
        for i in points:
            x = -5 + 5 * i
            p.line((x, y, surface(x, y)))
    p.line((-5, 80, 5))
    return cut_all(plate, p.swept)
