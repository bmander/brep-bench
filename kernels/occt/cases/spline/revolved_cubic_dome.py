"""A dome turned from a cubic Bézier curve. A dome made by turning a plane region a full turn
about the z axis. The region lies in the xz plane, on the side x ≥ 0, and is bounded by three
pieces: the segment of the z axis from (0, 0, 0) to (0, 0, 10); the segment of the x axis from
(0, 0, 0) to (9, 0, 0); and the cubic Bézier curve whose four control points are (9, 0, 0),
(12, 0, 4), (9, 0, 10) and (0, 0, 10).

The curve leaves the base's rim heading up and outward, along (3, 0, 4), so the dome overhangs
its flat round base (radius 9), bulging to 10.125 from the axis; it then curls in over the
top and meets the z axis at (0, 0, 10) heading straight at it, along −x. So the domed face closes
smoothly over a single point on the axis, with no spike, and the dome's only other face is the
flat disc it stands on."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp, gp_Pnt

from bench import Refused

CONTROL = [(9, 0, 0), (12, 0, 4), (9, 0, 10), (0, 0, 10)]


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()


def build():
    origin, rim, top = gp_Pnt(0, 0, 0), gp_Pnt(*CONTROL[0]), gp_Pnt(*CONTROL[-1])
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(origin, rim).Edge(), bezier(CONTROL),
                                   BRepBuilderAPI_MakeEdge(top, origin).Edge())
    if not wire.IsDone():
        raise Refused("the profile's three edges did not join into a wire")
    region = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()  # planar: in the xz plane
    return BRepPrimAPI_MakeRevol(region, gp.OZ_s(), 2 * math.pi).Shape()
