"""An oblate spheroid. A solid oblate spheroid centred on the origin, about the z axis: the
ellipsoid (x² + y²)/36 + z²/9 ≤ 1, of radius 6 at its equator in the plane z = 0 and reaching to z =
±3 at its poles. It is the ellipse x²/36 + z²/9 ≤ 1 of the xz plane turned a whole turn about the z
axis, its short axis the axis of the turn."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfEllipse
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

EQUATOR, POLE = 6, 3  # the radius at the equator, and the height of each pole


def build():
    # the ellipse x²/EQUATOR² + z²/POLE² = 1 in the xz plane: its major radius along the frame's
    # x, which is whichever of x and z the longer axis lies along
    if POLE > EQUATOR:
        frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0), gp_Dir(0, 0, 1))  # x along +z
    else:
        frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0))  # x along +x
    ellipse = gp_Elips(frame, max(EQUATOR, POLE), min(EQUATOR, POLE))
    south, north = gp_Pnt(0, 0, -POLE), gp_Pnt(0, 0, POLE)
    # its half with x ≥ 0, from pole to pole, closed by the axis
    arc = GC_MakeArcOfEllipse(ellipse, south, north, True).Value()
    if arc.Value(0.5 * (arc.FirstParameter() + arc.LastParameter())).X() < 0:
        arc = GC_MakeArcOfEllipse(ellipse, north, south, True).Value()
    half = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(north, south).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(half, True).Face()
    return BRepPrimAPI_MakeRevol(section, gp.OZ_s()).Shape()
