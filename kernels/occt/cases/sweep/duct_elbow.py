"""A hollow square duct elbow. A hollow square duct bent through a quarter turn. Its centreline is
an arc of radius 30 in the xy plane about the z axis, from (30, 0, 0) to (0, 30, 0).

At every point of the arc the section lies in the vertical plane through the z axis and that
point. It is a square 18 × 18 centred on the arc, two of its sides horizontal (at z = −9 and
z = 9) and two vertical (at 21 and 39 from the z axis), with a square hole 14 × 14 at its centre,
so the wall is 2 thick all round. The section is the same all the way round: the duct is that
hollow square, standing in the xz plane at x = 30, turned a quarter turn about the z axis, from +x
towards +y. Its two ends are flat, in the xz and yz planes."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Pnt

from bench import Refused

BEND_RADIUS = 30
OUTER = 18  # the section's side
INNER = 14  # the hole's side
TURN = 90  # degrees


def square(side):
    """A square of the given side in the xz plane, centred at (BEND_RADIUS, 0, 0)."""
    h = side / 2
    corners = [(BEND_RADIUS - h, -h), (BEND_RADIUS + h, -h), (BEND_RADIUS + h, h),
               (BEND_RADIUS - h, h)]
    poly = BRepBuilderAPI_MakePolygon()
    for x, z in corners:
        poly.Add(gp_Pnt(x, 0, z))
    poly.Close()
    return poly.Wire()


def build():
    # The hollow section: the outer square with the inner one as its hole (reversed, so the
    # face's material lies between them).
    maker = BRepBuilderAPI_MakeFace(square(OUTER))
    maker.Add(TopoDS.Wire(square(INNER).Reversed()))
    if not maker.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace could not make the hollow square")
    # The same section all the way round: a revolution about the z axis, from +x towards +y.
    revol = BRepPrimAPI_MakeRevol(maker.Face(), gp.OZ_s(), math.radians(TURN))
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol could not turn the section")
    return revol.Shape()
