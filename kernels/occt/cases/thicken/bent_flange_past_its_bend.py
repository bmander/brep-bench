"""A bent sheet thickened inside its bend past the bend's radius. The sheet of
`thicken/bent-flange`, thickened by 3 toward the bend's axis, more than the bend's radius of 2."""
from math import sqrt

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Pnt, gp_Vec

from thicken import thicken

FLOOR = 10
BEND_RADIUS = 2
WALL = 10
WIDTH = 8
THICKNESS = 3


def sheet():
    """The floor, the bend and the wall: the profile in the xz plane swept along +y."""
    r = BEND_RADIUS
    start, bend, top = gp_Pnt(-FLOOR, 0, 0), gp_Pnt(r, 0, r), gp_Pnt(r, 0, r + WALL)
    middle = gp_Pnt(r * sqrt(0.5), 0, r - r * sqrt(0.5))
    profile = BRepBuilderAPI_MakeWire()
    profile.Add(BRepBuilderAPI_MakeEdge(start, gp_Pnt(0, 0, 0)).Edge())
    arc = GC_MakeArcOfCircle(gp_Pnt(0, 0, 0), middle, bend).Value()
    profile.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    profile.Add(BRepBuilderAPI_MakeEdge(bend, top).Edge())
    return BRepPrimAPI_MakePrism(profile.Wire(), gp_Vec(0, WIDTH, 0)).Shape()


def build():
    # the profile runs from the floor to the wall, and swept along +y its faces' normals point
    # out of the bend, so inside is a negative distance
    return thicken(sheet(), -THICKNESS)
