"""A helical pipe of forty turns. A pipe of circular section, radius 1, following a helix about
the z axis, a coil spring. The helix has radius 10 and rises 5 per turn. It starts at (10, 0, 0),
turns right-handed (from +x towards +y) and makes forty whole turns, ending at (10, 0, 200). The
section stays square to the path, and the pipe's ends are flat."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GeomAbs import GeomAbs_Shape
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

HELIX_RADIUS = 10
PITCH = 5  # rise per turn
TURNS = 40
PIPE_RADIUS = 1
# The most pieces the helix in space may be approximated by: a few to each turn.
SEGMENTS = 10 * TURNS


def helix():
    """The helix as OCCT draws one: a straight line in the (angle, height) parameters of a
    cylinder. The cylinder's frame is gp.XOY, so angle 0 is +x and the angle turns towards +y;
    each turn, the line climbs 2π in angle and PITCH in height."""
    cylinder = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), HELIX_RADIUS)
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(2 * math.pi, PITCH))
    length = TURNS * math.hypot(2 * math.pi, PITCH)  # of the line, in the parameter plane
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, length).Edge()
    # The helix in space, from the line on the cylinder. Left to its default bound on the pieces
    # (MaxSegment 0), BuildCurves3d answers done with a B-spline that keeps the helix's ends but
    # not its turns: 1452 long, not 2521, and the pipe along it 4567 in volume, not 7921. Given
    # room for a few pieces a turn, it follows the helix.
    BRepLib.BuildCurves3d_s(edge, 1e-9, GeomAbs_Shape.GeomAbs_C1, 14, SEGMENTS)
    return edge


def build():
    path = helix()
    # The section where the helix starts: a circle centred on the helix, square to it.
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), PIPE_RADIUS)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps square to the helix
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the helix")
    if not pipe.MakeSolid():  # close the ends with flat discs
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
