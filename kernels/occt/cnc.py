"""What the `cnc/` cases share in OCCT: the cutters, each a solid of revolution drawn upright with
its tip at the origin, and a program's moves, each swept by `sweep.py` from OCCT's own sweeps of
faces (`slid` for a slide, `turned` for a turn), the tool's faces split where the move's contact
runs across them.

Where the contact lies follows from the cutter being a solid of revolution (SPEC.md, "A swept
solid"): its faces' normals at a point lie in the plane through the point and its axis.

- **A slide square to the axis**: a round face's normal is square to the slide on the plane through
  the axis square to the slide; its flat ends slide in their own planes.
- **A slide along the axis**: every face leads or trails all over, or slides along itself (a
  cylinder): nothing to split.
- **A slide aslant the axis**: a ball's contact is its great circle square to the slide (the plane
  through its centre square to the slide); a cylinder's, the two lines square to the slide's part
  square to the axis (the plane through the axis square to that part); its flat ends lead or trail
  all over. A torus's or a cone's contact is no plane's: refused.
- **A turn about a line along the axis, or meeting it**: the plane through both lines, where
  every point's velocity is square to the plane and every normal lies in it.

Moves wholly above or beside the block cut nothing, and are passed over (`reaching`); the rest
are cut from the block in Booleans of many tools each (`cut_all`).
"""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBndLib import BRepBndLib
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Bnd import Bnd_Box
from OCP.GC import GC_MakeArcOfCircle
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.TopAbs import TopAbs_FACE
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused
from sweep import found, moved, plane, shapes, slid, turned

ROOT2 = math.sqrt(0.5)


def meridian_solid(points, arcs):
    """The solid of revolution about the z axis of the closed meridian through `points` in the
    plane y = 0, (x, z) pairs, each step a line, or the arc through `arcs[k]` where it has one."""
    wire = BRepBuilderAPI_MakeWire()
    n = len(points)
    for k in range(n):
        a = gp_Pnt(points[k][0], 0, points[k][1])
        b = gp_Pnt(points[(k + 1) % n][0], 0, points[(k + 1) % n][1])
        if k in arcs:
            m = gp_Pnt(arcs[k][0], 0, arcs[k][1])
            wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(a, m, b).Value()).Edge())
        else:
            wire.Add(BRepBuilderAPI_MakeEdge(a, b).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp.OZ_s()).Shape()


def flat(radius, length):
    """A flat end mill: a cylinder from its tip, the middle of its flat end, `length` up."""
    return meridian_solid([(0, 0), (radius, 0), (radius, length), (0, length)], {})


def bull(radius, corner, length):
    """A bull-nose mill: a flat end mill whose end corner is rounded to `corner`."""
    flat_end = radius - corner
    middle = (flat_end + corner * ROOT2, corner - corner * ROOT2)
    return meridian_solid([(0, 0), (flat_end, 0), (radius, corner), (radius, length), (0, length)],
                          {1: middle})


def ball(radius, length):
    """A ball-end mill: a half ball whose lowest point is its tip, below a cylinder `length` up."""
    middle = (radius * ROOT2, radius - radius * ROOT2)
    return meridian_solid([(0, 0), (radius, radius), (radius, length), (0, length)], {0: middle})


def drill(radius, half, length):
    """A drill: a cone of half-angle `half` (degrees) from its tip to its side, `length` up."""
    shoulder = radius / math.tan(math.radians(half))
    return meridian_solid([(0, 0), (radius, shoulder), (radius, length), (0, length)], {})


def standing(tip, axis):
    """The motion that turns the upright z axis onto `axis` by the least turn and carries the origin
    to `tip`."""
    a = gp_Vec(*axis).Normalized()
    trsf = gp_Trsf()
    z = gp_Vec(0, 0, 1)
    k = z.Crossed(a)
    if k.Magnitude() > 1e-15:
        trsf.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(k)), z.Angle(a))
    elif a.Z() < 0:
        trsf.SetRotation(gp.OX_s(), math.pi)
    shift = gp_Trsf()
    shift.SetTranslation(gp_Vec(*tip))
    return shift.Multiplied(trsf)


class Program:
    """A cutter's program: the cutter drawn upright with its tip at the origin, where it stands
    (`trsf`), and the solids its moves sweep so far."""

    def __init__(self, cutter, tip=(0, 0, 0), axis=(0, 0, 1)):
        self.cutter = cutter
        self.trsf = standing(tip, axis)
        self.swept = []

    def tip(self):
        p = gp_Pnt(0, 0, 0).Transformed(self.trsf)
        return (p.X(), p.Y(), p.Z())

    def axis(self):
        d = gp_Dir(0, 0, 1).Transformed(self.trsf)
        return gp_Vec(d.X(), d.Y(), d.Z())

    def placed(self):
        return moved(self.cutter, self.trsf)

    def line(self, to):
        """A straight move of the tip to `to`, the cutter not turning."""
        tip = self.tip()
        d = gp_Vec(*[to[i] - tip[i] for i in range(3)])
        if d.Magnitude() <= 1e-12:
            return self
        a = self.axis()
        along = d.Dot(a)
        across = d.Subtracted(a.Multiplied(along))
        tool = self.placed()
        if across.Magnitude() <= 1e-12 * d.Magnitude():
            cuts = []
        elif abs(along) <= 1e-12 * d.Magnitude():
            cuts = [plane(tip, (d.X(), d.Y(), d.Z()))]
        else:
            cuts = self.aslant(tool, tip, a, d, across)
        self.swept += slid(tool, (d.X(), d.Y(), d.Z()), cuts)
        step = gp_Trsf()
        step.SetTranslation(d)
        self.trsf = step.Multiplied(self.trsf)
        return self

    def aslant(self, tool, tip, a, d, across):
        """Each round face of `tool` with the plane that splits it along its contact under a slide
        along `d` aslant its axis `a` (`across` its part square to the axis)."""
        cuts = []
        for face in found(tool, TopAbs_FACE):
            surface = BRepAdaptor_Surface(TopoDS.Face(face))
            kind = surface.GetType()
            if kind == GeomAbs_SurfaceType.GeomAbs_Sphere:
                c = surface.Sphere().Location()
                cuts.append((face, plane((c.X(), c.Y(), c.Z()), (d.X(), d.Y(), d.Z()))))
            elif kind == GeomAbs_SurfaceType.GeomAbs_Cylinder:
                cuts.append((face, plane(tip, (across.X(), across.Y(), across.Z()))))
            elif kind != GeomAbs_SurfaceType.GeomAbs_Plane:
                raise Refused("a cutter slid aslant its axis touches what it sweeps along no plane "
                              "on a torus or a cone, and OCCT sweeps no solid")
        return cuts

    def arc(self, centre, to, anticlockwise=True):
        """A move of the tip round the upright line through `centre` (x, y) to `to`, at one height,
        anticlockwise seen from above or clockwise; a whole circle where `to` is the tip."""
        tip = self.tip()
        a0 = math.atan2(tip[1] - centre[1], tip[0] - centre[0])
        a1 = math.atan2(to[1] - centre[1], to[0] - centre[0])
        sweep = (a1 - a0) if anticlockwise else (a0 - a1)
        sweep %= 2 * math.pi
        if sweep <= 1e-9:
            sweep = 2 * math.pi
        degrees = math.degrees(sweep) * (1 if anticlockwise else -1)
        return self.turn((centre[0], centre[1], 0), (0, 0, 1), degrees)

    def turn(self, origin, axis, degrees):
        """The cutter carried rigidly round the line through `origin` along `axis`, right-handed,
        by `degrees`: its axis along that line or meeting it, so that the plane through both is
        the contact."""
        tip = self.tip()
        k = gp_Vec(*axis).Normalized()
        r = gp_Vec(*[tip[i] - origin[i] for i in range(3)])
        velocity = k.Crossed(r)
        if velocity.Magnitude() <= 1e-12:
            # the tip on the line: the plane through the line and the cutter's axis
            velocity = k.Crossed(self.axis())
        cuts = [plane(tip, (velocity.X(), velocity.Y(), velocity.Z()))]
        line = gp_Ax1(gp_Pnt(*origin), gp_Dir(k))
        self.swept += turned(self.placed(), line, degrees, cuts)
        step = gp_Trsf()
        step.SetRotation(line, math.radians(degrees))
        self.trsf = step.Multiplied(self.trsf)
        return self


def rounded_loop(p, corners, radius, z):
    """The closed path round the polygon `corners` (anticlockwise, x and y) at height `z`, each
    corner rounded to `radius` by an arc tangent to both its sides, from the middle of its first
    side back to it."""
    n = len(corners)

    def unit(k):
        (ax, ay), (bx, by) = corners[k % n], corners[(k + 1) % n]
        length = math.hypot(bx - ax, by - ay)
        return ((bx - ax) / length, (by - ay) / length)

    for k in range(1, n + 1):
        vx, vy = corners[k % n]
        (ix, iy), (ox, oy) = unit(k - 1), unit(k)
        left = ix * oy - iy * ox > 0
        sx, sy = (-iy, ix) if left else (iy, -ix)
        a = (vx - radius * ix, vy - radius * iy, z)
        b = (vx + radius * ox, vy + radius * oy, z)
        p.line(a)
        p.arc((a[0] + radius * sx, a[1] + radius * sy), b, anticlockwise=left)
    (fx, fy), (gx, gy) = corners[0], corners[1 % n]
    p.line(((fx + gx) / 2, (fy + gy) / 2, z))


def reaching(block, pieces):
    """The pieces whose boxes meet the block's: the rest stand clear of it and cut nothing."""
    box = Bnd_Box()
    BRepBndLib.Add_s(block, box)
    kept = []
    for piece in pieces:
        b = Bnd_Box()
        BRepBndLib.Add_s(piece, b)
        if not b.IsOut(box):
            kept.append(piece)
    return kept


# How many tools one Boolean takes at once: a program's thousand in one exhausts memory.
BATCH = 48


def cut_all(block, pieces, what="what the cutters sweep"):
    """`block` less every one of `pieces`, cut in Booleans of up to `BATCH` tools each."""
    pieces = reaching(block, pieces)
    for start in range(0, len(pieces), BATCH):
        cutting = BRepAlgoAPI_Cut()
        cutting.SetArguments(shapes(block))
        cutting.SetTools(shapes(*pieces[start:start + BATCH]))
        cutting.Build()
        if not cutting.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined {what}")
        block = cutting.Shape()
    return block
