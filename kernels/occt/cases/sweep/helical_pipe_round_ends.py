"""A helical pipe with rounded ends. A pipe of circular section, radius 1, following a helix
about the z axis. The helix has radius 10 and rises 5 per turn. It starts at (10, 0, 0), turns
right-handed (from +x towards +y) and makes three whole turns. The section stays square to the
path. Each end is closed by a half ball of radius 1, as a ball rolled along the helix would
leave."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

HELIX_RADIUS = 10
PITCH = 5  # rise per turn
TURNS = 3
PIPE_RADIUS = 1


def helix():
    """The helix as OCCT draws one: a straight line in the (angle, height) parameters of a
    cylinder. The cylinder's frame is gp.XOY, so angle 0 is +x and the angle turns towards +y;
    each turn, the line climbs 2π in angle and PITCH in height."""
    cylinder = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), HELIX_RADIUS)
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(2 * math.pi, PITCH))
    length = TURNS * math.hypot(2 * math.pi, PITCH)  # of the line, in the parameter plane
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, length).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)  # the helix in space, from the line on the cylinder
    return edge


def build():
    path = helix()
    curve = BRepAdaptor_Curve(path)
    start, tangent = gp_Pnt(), gp_Vec()
    curve.D1(curve.FirstParameter(), start, tangent)
    end = curve.Value(curve.LastParameter())

    # The pipe: a circle square to the helix where it starts, swept along it.
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), PIPE_RADIUS)).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps square to the helix
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the helix")
    if not pipe.MakeSolid():
        raise Refused("the swept pipe did not close into a solid")
    body = pipe.Shape()

    # A ball on each end of the helix: half of it lies inside the pipe, and the other half
    # rounds the end off.
    for centre in (start, end):
        ball = BRepPrimAPI_MakeSphere(centre, PIPE_RADIUS).Shape()
        fuse = BRepAlgoAPI_Fuse(body, ball)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a ball on the end of the pipe")
        body = fuse.Shape()
    return body
