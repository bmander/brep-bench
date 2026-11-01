"""A spindle torus. A solid spindle torus centred on the origin, about the z axis: the circle
through the middle of its tube has radius 3, and the tube has radius 5, more than the middle
circle's. The solid is every point within 5 of that circle.

The tube is wider than its circle, so it crosses the axis: the torus's surface passes through
itself, and the solid is what the disc (x − 3)² + z² ≤ 25 of the xz plane fills as it turns a whole
turn about the z axis, counted once. That is the turn of the disc's part with x ≥ 0: an apple shape,
bounded only by the outer arc of the tube's circle, turned, which meets the axis at (0, 0, ±4) in a
dimple above and below. The inner part of the torus's surface, the lemon the tube sweeps where it
crosses the axis (`geometry/lemon`), lies inside the solid and is no part of its boundary."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Pnt

MAJOR, MINOR = 3, 5   # the middle circle's radius, and the tube's: the tube crosses the axis


def build():
    # what the disc (x − 3)² + z² ≤ 25 fills as it turns, counted once: the turn of its part with
    # x ≥ 0, the outer arc closed by the axis from (0, 0, 4) down to (0, 0, −4)
    pole = (MINOR ** 2 - MAJOR ** 2) ** 0.5
    north, south = gp_Pnt(0, 0, pole), gp_Pnt(0, 0, -pole)
    arc = GC_MakeArcOfCircle(south, gp_Pnt(MAJOR + MINOR, 0, 0), north).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(north, south).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
