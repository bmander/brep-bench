"""A hollow square duct narrowing round a bend. A hollow square duct that narrows as it bends
through a quarter turn. Its centreline is an arc of radius 30 in the xy plane about the z axis,
from (30, 0, 0) to (0, 30, 0).

At every point of the arc the section lies in the vertical plane through the z axis and that
point. It is a square centred on the arc, two of its sides horizontal and two vertical (one side
facing the z axis), with a square hole at its centre, so that the wall is 2 thick all round. The
section's outer half-width h runs linearly with the angle turned, from 9 at the start to 6 at the
end: at an angle θ from the xz plane, h = 9 − 3·θ/90°. So the outside of the section reaches
from 30 − h to 30 + h out from the z axis and from z = −h to z = h, and the hole is the square of
half-width h − 2: 18 × 18 with a hole 14 × 14 at the start, in the xz plane at x = 30, and
12 × 12 with a hole 8 × 8 at the end, in the yz plane at y = 30.

The sides of every section are straight, so each of the eight long faces is swept by one side of
the section: a straight segment that stays horizontal (or vertical) in its plane through the z
axis while its ends move along the spirals the section's corners trace. The two ends are flat."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Law import Law_Linear
from OCP.gp import gp, gp_Circ, gp_Pnt

from bench import Refused

BEND_RADIUS = 30
TURN = math.pi / 2
HALF_START, HALF_END = 9, 6  # the outside's half-width
WALL = 2


def square(half):
    """The section's square of half-width `half` where the arc starts: in the xz plane, centred
    at (BEND_RADIUS, 0, 0), its sides along x and z."""
    poly = BRepBuilderAPI_MakePolygon()
    for x, z in [(-half, -half), (half, -half), (half, half), (-half, half)]:
        poly.Add(gp_Pnt(BEND_RADIUS + x, 0, z))
    poly.Close()
    return poly.Wire()


def tapered(start, end, arc):
    """The solid swept by the square of half-width `start` round the arc, scaled about the arc
    linearly in its angle (the circle's parameter) until its half-width is `end`."""
    law = Law_Linear()
    law.Set(0, 1, TURN, end / start)
    pipe = BRepOffsetAPI_MakePipeShell(arc)
    pipe.SetMode(True)  # Frenet: the section stays in the plane through the z axis
    pipe.SetLaw(square(start), law, False, False)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the scaled square")
    if not pipe.MakeSolid():
        raise Refused("the swept square did not close into a solid")
    return pipe.Shape()


def build():
    # The arc: the circle about the z axis in the xy plane, its parameter the angle from +x.
    arc = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), BEND_RADIUS), 0, TURN).Edge()).Wire()
    outside = tapered(HALF_START, HALF_END, arc)
    # The hole, half-width h − 2, from end to end: its ends lie in the duct's.
    hole = tapered(HALF_START - WALL, HALF_END - WALL, arc)
    cut = BRepAlgoAPI_Cut(outside, hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
