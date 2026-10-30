"""A drafted star handle. A four-armed star handle, as on a radiator valve: a flat star outline in
the xz plane, extruded 12 along y with a draft.

**The outline**, in the plane y = 0 (the first number is x and the second z). Draw eight circles:

- four *tip* circles of radius 4, centred 33.1 from the origin on the axes: at (33.1, 0),
  (0, 33.1), (−33.1, 0) and (0, −33.1);
- four *fillet* circles of radius 3, centred 15.9 from the origin on the diagonals: at
  (±15.9/√2, ±15.9/√2), about (±11.243, ±11.243).

Each arm's two straight sides are tangent lines, each to the arm's tip circle and to the fillet
circle beside it, passing *between* the two circles (the tip circle on one side of the line, the
fillet circle on the other). For example the side from the tip at (33.1, 0) to the fillet circle
at (11.243, 11.243) touches the tip circle at about (33.8409, 3.9308) and the fillet circle at
about (10.6873, 8.2949). Going round, the outline is: the outer arc of a tip circle (convex, about
158.65° of it, crossing the axis), a straight side, the inner arc of a fillet circle (concave, the
part facing the origin, about 68.65°, crossing the diagonal), a straight side, the next tip's arc,
and so on round all four arms. The sides are tangent to the arcs where they meet. So the outline
measures 74.2 across opposite tips and 25.8 across opposite fillets (along a diagonal), and an
arm's two sides, carried on, would meet 54.6954 from the origin.

**The extrusion**, with a 2° draft: the solid runs from y = 0 to y = 12, and its section at y = h
is the outline moved h·tan 2° inward. Every straight side moves inward parallel to itself, each
tip arc's radius shrinks and each fillet arc's radius grows by h·tan 2°, the circles' centres
staying where they are. So the face at y = 0 is the whole outline, the face at y = 12 the outline
moved 12·tan 2° ≈ 0.419 inward, and the sides are planes (from the straight sides) and cones
(from the arcs), each leaning 2° in from the y direction."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.GccEnt import GccEnt_Position
from OCP.Geom2d import Geom2d_Circle
from OCP.Geom2dAdaptor import Geom2dAdaptor_Curve
from OCP.Geom2dGcc import Geom2dGcc_Lin2d2Tan, Geom2dGcc_QualifiedCurve
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2d, gp_Dir, gp_Pln, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

TIP_DISTANCE, TIP_RADIUS = 33.1, 4
FILLET_DISTANCE, FILLET_RADIUS = 15.9, 3
HEIGHT = 12
DRAFT = 2  # degrees
TOLERANCE = 1e-9


def polar(r, degrees):
    return (r * math.cos(math.radians(degrees)), r * math.sin(math.radians(degrees)))


def turned(point, degrees):
    """`point` of the outline's plane turned about the origin."""
    c, s = math.cos(math.radians(degrees)), math.sin(math.radians(degrees))
    return (c * point[0] - s * point[1], s * point[0] + c * point[1])


def xz(point):
    """A point (u, v) of the outline's plane in space: x = u, z = v, at y = 0."""
    return gp_Pnt(point[0], 0, point[1])


def first_side():
    """The side from the tip at (33.1, 0) to the fillet circle at 45°, as its points of contact
    (on the tip circle, on the fillet circle): of the lines tangent to both circles, the one that
    passes between them and stays in the arm's wedge, between the x axis and the diagonal."""
    tip, fillet = polar(TIP_DISTANCE, 0), polar(FILLET_DISTANCE, 45)
    circles = [Geom2dGcc_QualifiedCurve(
        Geom2dAdaptor_Curve(Geom2d_Circle(gp_Ax2d(gp_Pnt2d(*c), gp.DX2d_s()), r)),
        GccEnt_Position.GccEnt_unqualified)
        for c, r in [(tip, TIP_RADIUS), (fillet, FILLET_RADIUS)]]
    tangents = Geom2dGcc_Lin2d2Tan(*circles, TOLERANCE)
    if not tangents.IsDone():
        raise Refused("Geom2dGcc_Lin2d2Tan found no line tangent to a tip and a fillet circle")
    for i in range(1, tangents.NbSolutions() + 1):
        on_tip, on_fillet = gp_Pnt2d(), gp_Pnt2d()
        tangents.Tangency1(i, on_tip)
        tangents.Tangency2(i, on_fillet)
        (ax, ay), (bx, by) = points = [(p.X(), p.Y()) for p in (on_tip, on_fillet)]
        # Between the circles: their centres lie on opposite sides of the line.
        sides = [(bx - ax) * (cy - ay) - (by - ay) * (cx - ax) for cx, cy in (tip, fillet)]
        in_wedge = all(0 < math.degrees(math.atan2(y, x)) < 45 for x, y in points)
        if sides[0] * sides[1] < 0 and in_wedge:
            return points
    raise Refused("no line tangent to a tip and a fillet circle passes between them in the wedge")


def arc(start, through, end):
    return BRepBuilderAPI_MakeEdge(
        GC_MakeArcOfCircle(xz(start), xz(through), xz(end)).Value()).Edge()


def outline():
    on_tip, on_fillet = first_side()
    mirror = lambda p: (p[0], -p[1])  # the arm's other side, across its centre line
    wire = BRepBuilderAPI_MakeWire()
    for arm in range(4):
        a = 90 * arm
        tip_before, fillet_before = turned(mirror(on_tip), a), turned(mirror(on_fillet), a)
        tip_after, fillet_after = turned(on_tip, a), turned(on_fillet, a)
        next_fillet = turned(mirror(on_fillet), a + 90)
        # Counter-clockwise: out along one side, round the tip's outer arc (through its point on
        # the axis), in along the other side, round the fillet's inner arc (through its point on
        # the diagonal, facing the origin) to the next arm.
        wire.Add(BRepBuilderAPI_MakeEdge(xz(fillet_before), xz(tip_before)).Edge())
        wire.Add(arc(tip_before, polar(TIP_DISTANCE + TIP_RADIUS, a), tip_after))
        wire.Add(BRepBuilderAPI_MakeEdge(xz(tip_after), xz(fillet_after)).Edge())
        wire.Add(arc(fillet_after, polar(FILLET_DISTANCE - FILLET_RADIUS, a + 45), next_fillet))
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the outline")
    return wire.Wire()


def build():
    face = BRepBuilderAPI_MakeFace(outline(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, HEIGHT, 0)).Shape()
    # Draft every side face about the neutral plane y = 0, pulled along +y, so that the sides
    # lean inward as y grows. The caps are the faces whose normal is along y.
    draft = BRepOffsetAPI_DraftAngle(prism)
    pull = gp_Dir(0, 1, 0)
    neutral = gp_Pln(gp_Pnt(0, 0, 0), pull)
    faces = TopExp_Explorer(prism, TopAbs_FACE)
    while faces.More():
        f = TopoDS.Face(faces.Current())
        surface = BRepAdaptor_Surface(f)
        cap = (surface.GetType() == GeomAbs_SurfaceType.GeomAbs_Plane
               and abs(abs(surface.Plane().Axis().Direction().Y()) - 1) < TOLERANCE)
        if not cap:
            draft.Add(f, pull, math.radians(DRAFT), neutral)
            if not draft.AddDone():
                raise Refused("BRepOffsetAPI_DraftAngle declined a side face")
        faces.Next()
    draft.Build()
    if not draft.IsDone():
        raise Refused("BRepOffsetAPI_DraftAngle could not draft the sides")
    return draft.Shape()
