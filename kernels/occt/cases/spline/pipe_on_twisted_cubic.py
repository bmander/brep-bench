"""A pipe along a twisted cubic Bézier curve. A pipe of circular section, radius 8, swept along a
cubic Bézier curve that does not lie in a plane. The curve's four control points are (0, 0, 0),
(12, 16, 0), (26, 8, 0) and (24, 0, 16): it leaves the origin along (3, 4, 0), swings round to
the right and rises out of the xy plane to end at (24, 0, 16), arriving along (−1, −4, 8). Its
torsion never vanishes, and its radius of curvature is least, 10.28, a little past the middle of
the curve.

At every point of the curve the section is the disc of radius 8 centred on the curve, in the
plane square to the curve there. Since the section is a disc, how it turns about the curve does
not change the pipe. The pipe's two ends are flat discs, square to the curve at its two ends.

On the inside of the tightest part of the bend the wall's radius of curvature is only about
2.3; the wall never folds over itself and no two sections meet."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

CONTROL = [(0, 0, 0), (12, 16, 0), (26, 8, 0), (24, 0, 16)]
PIPE_RADIUS = 8


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
