"""A thin blade twisted through a full turn. A thin flat blade twisted through a whole turn: a solid lofted through one thin rectangle, turned
steadily as it rises, its four corners running along four helical guides.

The solid runs from z = 0 to z = 60. Its section at every height z is the rectangle 40 × 0.4,
centred on the z axis, turned about the z axis right-handed (from +x towards +y) by 6° for every
unit of height. So it begins as −20 ≤ x ≤ 20, −0.2 ≤ y ≤ 0.2 at z = 0; at z = 15 its long sides
are parallel to the y axis; at z = 30 it lies along x again; and at z = 60 it is back where it
began, having turned 360°.

Each corner of the rectangle runs along a helix of radius √(20² + 0.2²) about the z axis, rising 60
in a turn: the four guides. Each of the four walls is swept by one side of the rectangle, a
straight horizontal segment between two neighbouring helices. The two broad faces, 0.4 apart,
are swept by the long sides, each 0.2 from the axis; at their outer edges they are steep, the
helices there rising only 60 in a circumference of about 126. The two narrow faces, 0.4 wide, are
swept by the short sides, 20 from the axis. The ends are flat."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFill import BRepFill_TypeOfContact
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GeomAbs import GeomAbs_Shape
from OCP.gp import gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

HEIGHT = 60
HALF_LENGTH, HALF_WIDTH = 20, 0.2
TURN = 2 * math.pi  # how far it turns, right-handed about z, from foot to top


def helix():
    """The guide the corner (20, 0.2, 0) runs along: a line on the cylinder through the corners,
    its angle (the cylinder's u) growing steadily with height."""
    radius = math.hypot(HALF_LENGTH, HALF_WIDTH)
    cylinder = Geom_CylindricalSurface(
        gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), radius)
    du, dv = TURN / HEIGHT, 1  # per unit of height
    line = Geom2d_Line(gp_Pnt2d(math.atan2(HALF_WIDTH, HALF_LENGTH), 0), gp_Dir2d(du, dv))
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, HEIGHT * math.hypot(du, dv)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9, GeomAbs_Shape.GeomAbs_C2, 14, 0)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def build():
    a, b = HALF_LENGTH, HALF_WIDTH
    foot = BRepBuilderAPI_MakePolygon()
    for x, y in [(a, b), (-a, b), (-a, -b), (a, -b)]:
        foot.Add(gp_Pnt(x, y, 0))
    foot.Close()
    axis = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(0, 0, HEIGHT)).Edge()).Wire()

    # A sweep up the axis, the helix its auxiliary spine: at each height the rectangle is turned
    # so that its corner points at where the plane at that height meets the helix (no
    # curvilinear equivalence).
    pipe = BRepOffsetAPI_MakePipeShell(axis)
    pipe.SetMode(helix(), False, BRepFill_TypeOfContact.BRepFill_NoContact)
    pipe.Add(foot.Wire(), False, False)
    pipe.SetTolerance(1e-7, 1e-7, 1e-6)
    pipe.SetMaxDegree(14)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the rectangle along its guide")
    if not pipe.MakeSolid():
        raise Refused("the swept rectangle did not close into a solid")
    return pipe.Shape()
