"""A square bar bent along a cubic Bézier curve. A bar of square section, 20 × 20, bent along a
planar cubic Bézier curve in the xy plane. The curve's four control points are (0, 0, 0),
(16, 12, 0), (18, 26, 0) and (12, 34, 0): it leaves the origin along (4, 3, 0) and bends steadily
to the left, a quarter turn in all, to arrive at (12, 34, 0) heading along (−3, 4, 0). It bends
tightest at its end, where its radius of curvature is 15.

At every point of the curve the section is the square of side 20 centred on the curve, in the
plane square to the curve there, with two of its sides horizontal (at z = −10 and z = 10) and
two upright. So the bar's top and bottom are flat, and its two upright sides are traced by the
curve's offsets 10 to its left and 10 to its right in the xy plane. The two ends are flat
squares, square to the curve at its ends.

On the inside of the bend the upright side bends sharply where the bar ends: its radius of
curvature there is 15 − 10 = 5. It never folds over itself."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

CONTROL = [(0, 0, 0), (16, 12, 0), (18, 26, 0), (12, 34, 0)]
HALF = 10  # half the side of the square


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()


def build():
    path = bezier(CONTROL)
    # the section where the path starts: square to it, two sides horizontal
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    up = gp_Vec(0, 0, 1)
    across = up.Crossed(tangent).Normalized()  # horizontal, square to the path
    poly = BRepBuilderAPI_MakePolygon()
    for a, b in [(-1, -1), (1, -1), (1, 1), (-1, 1)]:
        poly.Add(start.Translated(across.Multiplied(a * HALF).Added(up.Multiplied(b * HALF))))
    poly.Close()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(gp_Dir(0, 0, 1))  # the binormal held to z: the section keeps two sides level
    pipe.Add(poly.Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the square along the curve")
    if not pipe.MakeSolid():
        raise Refused("the swept bar did not close into a solid")
    return pipe.Shape()
