"""A prolate spheroid hollowed, its cavity closed. The solid prolate spheroid (x² + y²)/9 + z²/36 ≤ 1,
the ellipse x²/9 + z²/36 ≤ 1 of the xz plane turned a whole turn about the z axis, its long axis the
axis of the turn, hollowed to a wall 1 thick with no opening: its surface stays as the outside, and
moved 1 inward along its normals (no ellipsoid) it is the inside, facing in, towards the void."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfEllipse
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from shell import hollow

EQUATOR, POLE = 3, 6  # the radius at the equator, and the height of each pole
THICKNESS = 1


def spheroid():
    # the ellipse x²/EQUATOR² + z²/POLE² = 1 in the xz plane, its major radius along +z
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0), gp_Dir(0, 0, 1)), POLE, EQUATOR)
    south, north = gp_Pnt(0, 0, -POLE), gp_Pnt(0, 0, POLE)
    # its half with x ≥ 0, from pole to pole, closed by the axis
    arc = GC_MakeArcOfEllipse(ellipse, south, north, True).Value()
    if arc.Value(0.5 * (arc.FirstParameter() + arc.LastParameter())).X() < 0:
        arc = GC_MakeArcOfEllipse(ellipse, north, south, True).Value()
    half = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(north, south).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(half, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()


def build():
    # OCCT builds this hollow (its own measures of it agree with the case's), but writes the
    # cavity's face as an OFFSET_SURFACE of the whole SURFACE_OF_REVOLUTION, bounded by a vertex
    # loop alone: the half ellipse it turns is not in the file, so the whole ellipse turned covers
    # the surface twice, and the offset of the half beyond the axis moves the other way.
    #
    # MakeThickSolid wants an opening; with none, the spheroid's shell and its shrunk copy's,
    # reversed
    return hollow(spheroid(), THICKNESS)
