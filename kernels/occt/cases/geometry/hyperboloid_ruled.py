"""A hyperboloid of one sheet as a ruled surface. The solid bounded by two flat discs and the
surface ruled between their rims, the rims turned a quarter turn against each other.

The two rims are circles of radius 5 about the z axis, one in the plane z = −5 and one in z = 5.
Join the point at angle θ on the lower circle, (5 cos θ, 5 sin θ, −5), by a straight line to the
point at angle θ + 90° on the upper one, (5 cos(θ + 90°), 5 sin(θ + 90°), 5), for every θ. Those
lines sweep the solid's side; the discs the circles bound are its ends.

The side is a hyperboloid of one sheet, x² + y² = 25/2 + z²/2: its waist, in the plane z = 0, has
radius 5/√2, where each line passes nearest the axis. Build the side as a ruled surface, as the
lines are given, rather than as a hyperbola turned."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_Sewing)
from OCP.BRepFill import BRepFill
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 5   # each rim
H = 5        # the rims are at z = ±H
TWIST = 90   # degrees: the upper end of each line is this far round from its lower end


def rim(z, start):
    """The circle of radius RADIUS about z at height z, its parameter 0 at angle `start`."""
    a = math.radians(start)
    frame = gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1), gp_Dir(math.cos(a), math.sin(a), 0))
    return BRepBuilderAPI_MakeEdge(gp_Circ(frame, RADIUS)).Edge()


def build():
    lower, upper = rim(-H, 0), rim(H, TWIST)
    # BRepFill.Face joins the points of the two edges at the same parameter by lines: the point at
    # θ below to the one at θ + TWIST above
    side = BRepFill.Face_s(lower, upper)
    ends = [BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(e).Wire()).Face()
            for e in (lower, upper)]
    sewing = BRepBuilderAPI_Sewing(1e-7)
    for face in [side] + ends:
        sewing.Add(face)
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the ruled side and its ends")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the sewn shell")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid
