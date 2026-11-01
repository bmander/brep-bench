"""A lemon: a circular segment turned about its chord. The inner part of a spindle torus: a lemon,
pointed at both ends. In the xz plane take the circle of radius 5 centred at (−3, 0, 0), and the
part of its disc with x ≥ 0: a circular segment, cut off by the z axis as its chord, from (0, 0, −4)
to (0, 0, 4), and reaching out to (2, 0, 0). Turn it a whole turn about the z axis.

The solid is the region the tube of `geometry/spindle-torus` (middle circle radius 3, tube radius 5)
sweeps twice: every point within 5 of every point of the circle of radius 3 about the z axis in the
plane z = 0. Its surface is the segment's arc turned, which meets the axis at (0, 0, ±4) at an
angle, so the lemon comes to a sharp point at each end, a cone-like apex where the surface has no
tangent plane, and is widest, radius 2, at z = 0."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Pnt

CENTRE, RADIUS = -3, 5   # the circle's centre on the x axis, and its radius


def build():
    # the segment of the disc about (−3, 0, 0) with x ≥ 0: its arc through (2, 0, 0), closed by
    # its chord on the z axis
    tip = (RADIUS ** 2 - CENTRE ** 2) ** 0.5
    north, south = gp_Pnt(0, 0, tip), gp_Pnt(0, 0, -tip)
    arc = GC_MakeArcOfCircle(south, gp_Pnt(CENTRE + RADIUS, 0, 0), north).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(north, south).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
