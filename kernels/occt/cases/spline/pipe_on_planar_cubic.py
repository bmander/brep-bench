"""A fat pipe round a cubic Bézier bend. A pipe of circular section, radius 20, swept along a
planar cubic Bézier curve in the xy plane. The curve's four control points are (0, 0, 0),
(12, −16, 0), (32, −16, 0) and (44, 0, 0): it leaves the origin heading down and to the right,
along (3, −4, 0), dips to y = −12 at x = 22 and climbs back to (44, 0, 0), arriving along
(3, 4, 0). The curve is a polynomial cubic, not a circular arc: its radius of curvature runs from
24 at its lowest point to 37.5 at its ends.

At every point of the curve the section is the disc of radius 20 centred on the curve, in the
plane square to the curve there. The pipe's two ends are flat discs, square to the curve at its
two ends.

The radius is close to the curve's least radius of curvature, so on the inside of the bend the
wall curves tightly, with a radius of curvature of only 24 − 20 = 4 at the lowest point; it
never folds over itself and no two sections meet."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

CONTROL = [(0, 0, 0), (12, -16, 0), (32, -16, 0), (44, 0, 0)]
PIPE_RADIUS = 20


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()


def build():
    path = bezier(CONTROL)
    # the section where the path starts: a circle centred on it, square to it
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), PIPE_RADIUS)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section stays square to the curve
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the curve")
    if not pipe.MakeSolid():  # close the ends with flat discs
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
