"""A cap of a hyperboloid of two sheets. The solid inside one sheet of a hyperboloid of two sheets
about the z axis, below the plane z = 6: every point with z ≥ 3√(1 + (x² + y²)/16) and z ≤ 6.

Its curved side is the upper sheet z²/9 − (x² + y²)/16 = 1, which comes to its vertex at (0, 0, 3),
smooth and square to the axis there, and widens to a circle of radius 4√3 in the plane z = 6, where
a flat disc closes it. It is the hyperbola z²/9 − x²/16 = 1 of the xz plane (its branch with z > 0,
from its vertex to x = 4√3) turned a whole turn about the z axis, which it meets at its vertex: a
surface of revolution with a pole there."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfHyperbola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Hypr, gp_Pnt

A, C = 4, 3   # z²/C² − (x² + y²)/A² = 1
H = 6         # the top, a flat disc at z = H


def build():
    # the hyperbola z²/C² − x²/A² = 1 of the xz plane: its major axis (C) along +z, its minor (A)
    # along the frame's y, which is +x when the frame's normal is −y
    branch = gp_Hypr(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(0, 0, 1)), C, A)
    vertex, rim = gp_Pnt(0, 0, C), gp_Pnt(A * (H ** 2 / C ** 2 - 1) ** 0.5, 0, H)
    arc = GC_MakeArcOfHyperbola(branch, vertex, rim, True).Value()
    top = gp_Pnt(0, 0, H)
    # the section: the arc from the vertex out to the rim, the top's radius, the axis back down
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(rim, top).Edge(),
                                   BRepBuilderAPI_MakeEdge(top, vertex).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
