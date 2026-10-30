"""A turned shaft with a rounded shoulder. A stepped shaft on the z axis, the solid swept by
turning a section in the xz plane a full turn about the z axis. In the xz plane (the first number
is x and the second z), the section's outline runs:

- from (0, 0) out to (20, 0), the end of the thick part;
- from (20, 0) up to (20, 60), the thick part's side;
- from (20, 60) in to (16, 60), the shoulder;
- along a quarter circle of radius 4 centred at (16, 64), from (16, 60) to (12, 64), bulging
  toward the corner (12, 60) (its middle is at (16 − 2√2, 64 − 2√2));
- from (12, 64) up to (12, 160), the thin part's side;
- from (12, 160) in to (0, 160), the end of the thin part;
- from (0, 160) down the axis back to (0, 0).

So it is a cylinder of radius 20 from z = 0 to z = 60 and a cylinder of radius 12 from z = 60 to
z = 160, with the inside corner where the shoulder meets the thin part rounded to radius 4: the
round is tangent to the shoulder face at radius 16 and to the thin part's side at z = 64."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Pnt

from bench import Refused

ROUND = 4
ROUND_CENTRE = (16, 64)
HALF = ROUND / math.sqrt(2)


def xz(x, z):
    return gp_Pnt(x, 0, z)


def build():
    wire = BRepBuilderAPI_MakeWire()
    for a, b in [((0, 0), (20, 0)), ((20, 0), (20, 60)), ((20, 60), (16, 60))]:
        wire.Add(BRepBuilderAPI_MakeEdge(xz(*a), xz(*b)).Edge())
    cx, cz = ROUND_CENTRE
    arc = GC_MakeArcOfCircle(xz(16, 60), xz(cx - HALF, cz - HALF), xz(12, 64)).Value()
    wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    for a, b in [((12, 64), (12, 160)), ((12, 160), (0, 160)), ((0, 160), (0, 0))]:
        wire.Add(BRepBuilderAPI_MakeEdge(xz(*a), xz(*b)).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the section")
    section = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    revol = BRepPrimAPI_MakeRevol(section, gp.OZ_s())
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol declined the section")
    return revol.Shape()
