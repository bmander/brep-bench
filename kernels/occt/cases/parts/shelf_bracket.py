"""A shelf bracket with a rounded inside corner. An L-shaped plate bracket, 10 thick: its outline
lies in the xz plane and is extruded from y = −5 to y = 5. In the xz plane (the first number is x
and the second z), the outline runs:

- from (0, 0) to (20, 0), the foot of the post;
- from (20, 0) up to (20, 92), the inside of the post;
- along a quarter circle of radius 8 centred at (28, 92), from (20, 92) to (28, 100), bulging
  toward the corner (20, 100) (its middle is at (28 − 4√2, 92 + 4√2));
- from (28, 100) to (100, 100), the underside of the arm;
- from (100, 100) up to (100, 120), the end of the arm;
- from (100, 120) back to (0, 120), the top;
- from (0, 120) down to (0, 0), the back of the post.

So it is a post 20 wide and 120 tall with an arm 20 deep reaching out to x = 100 along its top,
the inside corner between them rounded to radius 8, the round tangent to the post at z = 92 and
to the arm at x = 28."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

ROUND = 8
ROUND_CENTRE = (28, 92)
HALF = ROUND / math.sqrt(2)
NEAR, FAR = -5, 5  # y


def xz(x, z):
    return gp_Pnt(x, NEAR, z)


def build():
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(xz(0, 0), xz(20, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(xz(20, 0), xz(20, 92)).Edge())
    cx, cz = ROUND_CENTRE
    arc = GC_MakeArcOfCircle(xz(20, 92), xz(cx - HALF, cz + HALF), xz(28, 100)).Value()
    wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    for a, b in [((28, 100), (100, 100)), ((100, 100), (100, 120)), ((100, 120), (0, 120)),
                 ((0, 120), (0, 0))]:
        wire.Add(BRepBuilderAPI_MakeEdge(xz(*a), xz(*b)).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the outline")
    outline = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakePrism(outline, gp_Vec(0, FAR - NEAR, 0)).Shape()
