"""A square tapering through a half turn on four spiral guides. A square column that tapers as it twists: a solid lofted from a large square at its foot to a
small one at its top, turning half a turn on the way, its four corners running along four spiral
guides.

The solid runs from z = 0 to z = 40. Its section at every height z is a square in the plane at
that height, centred on the z axis, of side 30 − z/2, and turned about the z axis, right-handed
(from +x towards +y), by 4.5° for every unit of height. So it begins as the square
−15 ≤ x ≤ 15, −15 ≤ y ≤ 15 at z = 0; at z = 10 it is 25 across and has turned 45°, its corners on
the axes; and it ends at z = 40 as the square −5 ≤ x ≤ 5, −5 ≤ y ≤ 5, having turned 180°.

Each corner of the square runs along a conical spiral, the four guides: the corner that starts at
(15, 15, 0) is, at height z, (15 − z/4)√2 from the z axis, at 45° + 4.5°·z from the +x direction.
Each of the four walls is swept by one side of the square, a straight horizontal segment between
two neighbouring guides, so the walls are twisted ruled surfaces. The ends are flat: the square of
side 30 at z = 0 and the square of side 10 at z = 40."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFill import BRepFill_TypeOfContact
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_ConicalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GeomAbs import GeomAbs_Shape
from OCP.Law import Law_Linear
from OCP.gp import gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

HEIGHT = 40
HALF_START, HALF_END = 15, 5  # the square's half-side at the foot and the top
TURN = math.pi  # how far it turns, right-handed about z, from foot to top


def spiral():
    """The guide the corner (15, 15, 0) runs along: a line on the cone about the z axis through
    the circle of the corners at the foot, the corners' distance from the axis shrinking with
    height as the cone does, their angle (the cone's u) growing with it."""
    slope = (HALF_END - HALF_START) * math.sqrt(2) / HEIGHT  # the corners' distance per height
    angle = math.atan(slope)  # the cone's half-angle, negative: it narrows as it rises
    cone = Geom_ConicalSurface(gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), angle,
                               HALF_START * math.sqrt(2))
    # on the cone, height z is v·cos(angle) along a generator, and the turn is u
    du, dv = TURN / HEIGHT, 1 / math.cos(angle)  # per unit of height
    run = math.hypot(du, dv)
    line = Geom2d_Line(gp_Pnt2d(math.pi / 4, 0), gp_Dir2d(du, dv))
    edge = BRepBuilderAPI_MakeEdge(line, cone, 0, HEIGHT * run).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9, GeomAbs_Shape.GeomAbs_C2, 14, 0)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def build():
    h = HALF_START
    foot = BRepBuilderAPI_MakePolygon()
    for x, y in [(h, h), (-h, h), (-h, -h), (h, -h)]:
        foot.Add(gp_Pnt(x, y, 0))
    foot.Close()
    axis = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(0, 0, HEIGHT)).Edge()).Wire()

    # A sweep up the axis, the spiral its auxiliary spine: at each height the square is turned
    # so that its corner points at where the plane at that height meets the spiral (no
    # curvilinear equivalence), and scaled about the axis linearly with height, from 1 at the
    # foot to a third at the top.
    taper = Law_Linear()
    taper.Set(0, 1, HEIGHT, HALF_END / HALF_START)
    pipe = BRepOffsetAPI_MakePipeShell(axis)
    pipe.SetMode(spiral(), False, BRepFill_TypeOfContact.BRepFill_NoContact)
    pipe.SetLaw(foot.Wire(), taper, False, False)
    pipe.SetTolerance(1e-7, 1e-7, 1e-6)
    pipe.SetMaxDegree(14)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the square along its guide")
    if not pipe.MakeSolid():
        raise Refused("the swept square did not close into a solid")
    return pipe.Shape()
