"""A hyperboloid of one sheet. The solid inside a hyperboloid of one sheet about the z axis, between
the planes z = −6 and z = 6: every point with x² + y² ≤ 16(1 + z²/36) and −6 ≤ z ≤ 6. Its waist, in
the plane z = 0, is a circle of radius 4; it flares to a circle of radius 4√2 at each end, which is
flat.

Its side is the hyperbola x²/16 − z²/36 = 1 of the xz plane (its branch with x > 0, between z = −6
and z = 6) turned a whole turn about the z axis. Its curvature is negative everywhere: it curves
away from its axis along each meridian and round it along each parallel."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfHyperbola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Hypr, gp_Pnt

A, C = 4, 6   # the waist's radius, and the hyperbola's other semi-axis
H = 6         # the ends are at z = ±H


def build():
    # the hyperbola x²/A² − z²/C² = 1 of the xz plane: its major axis (A) along +x, its minor (C)
    # along the frame's y, which is +z when the frame's normal is −y
    branch = gp_Hypr(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), A, C)
    rim = A * (1 + H ** 2 / C ** 2) ** 0.5
    low, high = gp_Pnt(rim, 0, -H), gp_Pnt(rim, 0, H)
    arc = GC_MakeArcOfHyperbola(branch, low, high, True).Value()
    # the section: the arc, the top's radius, the axis, the floor's radius
    top, bottom = gp_Pnt(0, 0, H), gp_Pnt(0, 0, -H)
    wire = BRepBuilderAPI_MakeWire()
    for edge in (BRepBuilderAPI_MakeEdge(arc).Edge(), BRepBuilderAPI_MakeEdge(high, top).Edge(),
                 BRepBuilderAPI_MakeEdge(top, bottom).Edge(),
                 BRepBuilderAPI_MakeEdge(bottom, low).Edge()):
        wire.Add(edge)
    section = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
