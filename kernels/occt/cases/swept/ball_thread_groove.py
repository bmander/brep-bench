"""A rod threaded by a ball. A rod, the cylinder of radius 5 about the z axis from z = 0 to
z = 20, with a round-bottomed helical groove cut round it by a ball of radius 1. The ball's centre
follows a helix of radius 5 about the z axis, so it runs on the rod's surface: it starts at
(5, 0, −2), turns right-handed (from +x towards +y) and rises 4 each turn, for six whole turns,
ending at (5, 0, 22). The ball is clear of the rod at both ends. Cut from the rod everything the
ball passes through.

Neighbouring turns of the groove do not meet: they are 4 apart and the ball is 2 across."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

ROD_RADIUS = 5
ROD_LENGTH = 20
HELIX_RADIUS = 5
HELIX_START = -2  # z of the ball's centre where it starts, at angle 0 (on +x)
PITCH = 4  # rise per turn
TURNS = 6
BALL_RADIUS = 1


def helix():
    """The helix as OCCT draws one: a straight line in the (angle, height) parameters of a
    cylinder. The cylinder's frame stands at z = −2 with its x direction along +x, so angle 0 is
    +x and the angle turns towards +y; each turn, the line climbs 2π in angle and PITCH in
    height."""
    frame = gp_Ax3(gp_Pnt(0, 0, HELIX_START), gp.DZ_s(), gp.DX_s())
    cylinder = Geom_CylindricalSurface(frame, HELIX_RADIUS)
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

    # OCCT has no sweep of a solid along a path: BRepOffsetAPI_MakePipe takes no solid ("profile
    # contains solids"), and given the ball's faces it stops in BRepFill_Sweep (Standard_OutOfRange)
    # whatever the path. So the ball's sweep is built from what elementary geometry says it is.
    # A point p of the ball, centre c, moving with the ball has velocity v + ω × (p − c), and
    # (ω × (p − c))·(p − c) = 0, so it runs along the sphere exactly where v·(p − c) = 0: on the
    # great circle square to the path. What the ball passes through is therefore the tube those
    # circles sweep, the circle of radius 1 kept square to the helix, with the ball where it
    # starts and where it ends. The helix turns no tighter than radius 5 + (4/2π)²/5 ≈ 5.08, far
    # more than the ball's, and its turns are 4 apart, so the tube neither folds nor meets itself.
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), BALL_RADIUS)).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    # corrected Frenet, as for any path: the section is a circle, so every frame that keeps it
    # square to the helix sweeps the same tube
    pipe.SetMode(False)
    # the tube's surface is a B-spline fitted by the sweep: ask for it within 1e-8, not the 1e-4
    # it settles for by default
    pipe.SetTolerance(1e-8, 1e-8, 1e-6)
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the helix")
    if not pipe.MakeSolid():
        raise Refused("the swept tube did not close into a solid")

    # Cut the sweep from the rod a piece at a time, the tube and then the ball at each end, as
    # holes are cut one after another. Each ball touches the tube all round the circle at its end;
    # cut one after another, the pieces that touch are never fused to each other. (Both balls are
    # clear of the rod, so their cuts leave it as it is.)
    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    pieces = [("the tube", pipe.Shape())] + [
        (f"the ball at the {which}", BRepPrimAPI_MakeSphere(centre, BALL_RADIUS).Shape())
        for which, centre in [("start", start), ("end", end)]]
    for name, piece in pieces:
        cut = BRepAlgoAPI_Cut(rod, piece)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined {name}")
        rod = cut.Shape()
    return rod
