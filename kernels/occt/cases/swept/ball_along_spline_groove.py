"""A groove carried along a spline by a ball. A block, the box from (−6, −6, −8) to (36, 26, 0),
with a groove cut into its top face by a ball of radius 3 carried along a curve. The ball's centre
follows the cubic Bézier curve in the top face (z = 0) whose control points are (0, 0, 0),
(10, 0, 0), (20, 20, 0) and (30, 20, 0), from its first end to its last. Cut from the block
everything the ball passes through.

The groove is half of the tube of radius 3 about the curve, closed at each end by a quarter of the
ball. The curve's tightest turn has a radius of about 7.5, more than the ball's, so the tube does
not fold on itself."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

BLOCK_LOW, BLOCK_HIGH = (-6, -6, -8), (36, 26, 0)
CONTROL = [(0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0)]
BALL_RADIUS = 3


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, p in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()


def build():
    path = bezier(CONTROL)
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
    # circles sweep, the circle of radius 3 kept square to the curve, with the ball where it
    # starts and where it ends; the curve never turns tighter than the ball, so the tube does not
    # fold.
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), BALL_RADIUS)).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    # corrected Frenet: the curve has an inflection, where the Frenet frame would flip over
    pipe.SetMode(False)
    # the tube's surface is a B-spline fitted by the sweep: ask for it within 1e-8, not the 1e-4
    # it settles for by default
    pipe.SetTolerance(1e-8, 1e-8, 1e-6)
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the curve")
    if not pipe.MakeSolid():
        raise Refused("the swept tube did not close into a solid")

    # Cut the sweep from the block a piece at a time, the tube and then the ball at each end, as
    # holes are cut one after another. Each ball touches the tube all round the circle at its end;
    # cut one after another, the pieces that touch are never fused to each other.
    block = BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK_LOW), gp_Pnt(*BLOCK_HIGH)).Shape()
    pieces = [("the tube", pipe.Shape())] + [
        (f"the ball at the {which}", BRepPrimAPI_MakeSphere(centre, BALL_RADIUS).Shape())
        for which, centre in [("start", start), ("end", end)]]
    for name, piece in pieces:
        cut = BRepAlgoAPI_Cut(block, piece)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined {name}")
        block = cut.Shape()
    return block
