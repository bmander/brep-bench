"""Two spherical sectors ten times larger sharing an apex. Each the part of a ball of radius 100
centred on the origin within 45° of an axis: the first about +z, the second the first turned 60°
about the y axis. The solid common to both."""
from math import cos, radians, sin

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt, gp_Trsf

from bench import Refused

RADIUS = 100
HALF_ANGLE = radians(45)
TURN = radians(60)


def sector():
    """The sector about +z: its section in the xz plane — the ruling from the apex to the rim, the
    arc of the sphere up to the axis, and the axis back down — turned a whole turn about z."""
    apex = gp_Pnt(0, 0, 0)
    rim = gp_Pnt(RADIUS * sin(HALF_ANGLE), 0, RADIUS * cos(HALF_ANGLE))
    middle = gp_Pnt(RADIUS * sin(HALF_ANGLE / 2), 0, RADIUS * cos(HALF_ANGLE / 2))
    top = gp_Pnt(0, 0, RADIUS)
    wire = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(apex, rim).Edge(),
        BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(rim, middle, top).Value()).Edge(),
        BRepBuilderAPI_MakeEdge(top, apex).Edge()).Wire()
    section = BRepBuilderAPI_MakeFace(wire).Face()
    return BRepPrimAPI_MakeRevol(section, gp_Ax1(apex, gp.DZ_s())).Shape()


def build():
    first = sector()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DY_s()), TURN)
    second = BRepBuilderAPI_Transform(first, turn, True).Shape()
    # At a tenth of the size OCCT finds this common part; here, from 50 or so up, it reports
    # itself done and gives an empty compound.
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two sectors")
    return common.Shape()
