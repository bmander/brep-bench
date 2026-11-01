"""A paraboloid. A solid paraboloid of revolution about the z axis, its vertex at the origin, filled
up to the plane z = 8: every point with x² + y² ≤ 8z and z ≤ 8. Its curved side is the paraboloid x²
+ y² = 8z, of focal length 2, which is smooth at its vertex and widens to a circle of radius 8 at z
= 8, where a flat disc closes it.

It is the parabola x² = 8z of the xz plane (from its vertex to x = 8) turned a whole turn about the
z axis, which it meets at its vertex."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfParabola
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Parab, gp_Pnt

FOCAL = 2     # x² + y² = 4·FOCAL·z
H = 8         # the top, a flat disc at z = H


def build():
    # the parabola x² = 8z of the xz plane: gp_Parab opens along its frame's x (here +z), and its
    # frame's y is +x when its normal is −y
    parabola = gp_Parab(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(0, 0, 1)), FOCAL)
    vertex, rim, top = gp_Pnt(0, 0, 0), gp_Pnt((4 * FOCAL * H) ** 0.5, 0, H), gp_Pnt(0, 0, H)
    arc = GC_MakeArcOfParabola(parabola, vertex, rim, True).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(rim, top).Edge(),
                                   BRepBuilderAPI_MakeEdge(top, vertex).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
