"""What the OCCT implementations of the swept cases share: everything a solid passes through as it
moves, built from OCCT's sweeps of faces.

OCCT sweeps no solid: BRepPrimAPI_MakePrism and MakeRevol answer "Solids are not Processed", and
BRepOffsetAPI_MakePipe "profile contains solids". But each sweeps a face, to the solid that face
passes through, and what a solid passes through follows from what its faces do, by elementary
geometry (SPEC.md, "A swept solid"):

- A point the moving solid covers at some instant is either inside it where it starts, or enters
  it later, and then it enters across the solid's boundary where the boundary moves towards it:
  where the face's outward normal leads the motion (makes an acute angle with the velocity). So
  everything the solid passes through is the solid where it starts, with what its *leading*
  faces sweep.
- A face leads on one side of its *contact*, the curve along which its normal is square to the
  velocity, and trails on the other. Split along its contact, each piece either leads all over or
  trails all over, and a leading piece never moves back over where it has been, so it sweeps to a
  solid without folding. Each case names the surfaces that split its tool's faces along their
  contact, from the elementary geometry of those faces, and says why beside them. `leads`
  checks that every piece leads or trails all over, so that a contact a case misses is a
  failure, not a wrong solid.
- A face the motion keeps in itself (a flat face sliding or turning in its own plane, a cylinder
  sliding along its axis, a surface of revolution turning about its own axis) neither leads nor
  trails, and sweeps nothing of its own.

`slid`, `turned` and `threaded` (`screwed`) give the solid where the tool starts and what its
leading faces sweep, for a slide (BRepPrimAPI_MakePrism), a turn about a fixed axis
(BRepPrimAPI_MakeRevol) and a turn about the z axis while rising along it
(BRepOffsetAPI_MakePipeShell, along the path a point of the tool takes); `united` makes them one
solid, and `cut` takes them from a block one after another. `flat_contacts_turned` and
`flat_contacts_screwed` give the planes along which a turn and a screw run along a tool's flat
faces.
"""
import math

from OCP.BOPAlgo import BOPAlgo_GlueEnum
from OCP.BRep import BRep_Builder, BRep_Tool
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import (BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse, BRepAlgoAPI_Section,
                             BRepAlgoAPI_Splitter)
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeSolid,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Sewing,
                                BRepBuilderAPI_Transform)
from OCP.BRepClass import BRepClass_FaceClassifier
from OCP.BRepFeat import BRepFeat_SplitShape
from OCP.BRepGProp import BRepGProp, BRepGProp_Face
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.BRepTools import BRepTools, BRepTools_ReShape
from OCP.GProp import GProp_GProps
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_BezierCurve
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.ShapeAnalysis import ShapeAnalysis_FreeBounds
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.TopAbs import (TopAbs_EDGE, TopAbs_FACE, TopAbs_IN, TopAbs_SHELL, TopAbs_VERTEX,
                        TopAbs_WIRE)
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS, TopoDS_Compound
from OCP.collections import Array1_gp_Pnt2d, List_TopoDS_Shape
from OCP.gp import gp, gp_Ax3, gp_Dir, gp_Pln, gp_Pnt, gp_Pnt2d, gp_Trsf, gp_Vec

from bench import Refused

GeomAbs_Plane = GeomAbs_SurfaceType.GeomAbs_Plane
GeomAbs_SurfaceOfRevolution = GeomAbs_SurfaceType.GeomAbs_SurfaceOfRevolution

# How far a splitting plane reaches either way from the point it is placed by: past any tool here.
REACH = 100
# How a face's normal must stand to the velocity, as a cosine, for it to count as leading or
# trailing at a point; nearer square than this, the motion runs along the face there.
SQUARE = 1e-9
# How many points a side of the grid of a face's parameters samples it at.
SAMPLES = 9
# The most a thread is swept at once, in degrees (see `threaded`).
SPAN = 180


def shapes(*items):
    """An OCCT list of shapes, for the Boolean operations' arguments and tools."""
    listed = List_TopoDS_Shape()
    for item in items:
        listed.Append(item)
    return listed


def found(shape, kind):
    """The sub-shapes of `shape` of a kind (TopAbs_FACE, TopAbs_SOLID, ...), each once."""
    seen = []
    explorer = TopExp_Explorer(shape, kind)
    while explorer.More():
        current = explorer.Current()
        if not any(current.IsSame(s) for s in seen):
            seen.append(current)
        explorer.Next()
    return seen


def plane(point, normal, reach=REACH):
    """A square of the plane through `point` square to `normal`, reaching `reach` either way: a
    face to split a tool with."""
    return BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(*point), gp_Dir(*normal)),
                                   -reach, reach, -reach, reach).Face()


def moved(shape, trsf):
    return BRepBuilderAPI_Transform(shape, trsf, True).Shape()


def shift(shape, vector):
    """`shape` moved by `vector` (x, y, z)."""
    trsf = gp_Trsf()
    trsf.SetTranslation(gp_Vec(*vector))
    return moved(shape, trsf)


def turn(shape, axis, degrees):
    """`shape` turned by `degrees` about `axis` (a gp_Ax1), right-handed."""
    trsf = gp_Trsf()
    trsf.SetRotation(axis, math.radians(degrees))
    return moved(shape, trsf)


def screw(shape, lead, degrees):
    """`shape` carried along a screw about the z axis: turned by `degrees`, right-handed, and
    raised `lead` for every whole turn."""
    trsf = gp_Trsf()
    trsf.SetRotation(gp.OZ_s(), math.radians(degrees))
    rise = gp_Trsf()
    rise.SetTranslation(gp_Vec(0, 0, lead * degrees / 360))
    return moved(shape, rise.Multiplied(trsf))


def split(tool, cuts):
    """The faces of `tool`, split by `cuts`: faces that split every face they cross, or pairs
    (face, cutter), a face of the tool and a face that splits it alone."""
    own = [c for c in cuts if isinstance(c, tuple)]
    if own:
        tool = split_each(tool, own)
    cuts = [c for c in cuts if not isinstance(c, tuple)]
    if not cuts:
        return found(tool, TopAbs_FACE)
    return found(split_shell(tool, cuts), TopAbs_FACE)


def split_each(tool, pairs):
    """`tool` with each face of `pairs` split where its cutter crosses it, and no other face
    (BRepFeat_SplitShape, which keeps the edges the split faces share with their neighbours)."""
    splitting = BRepFeat_SplitShape(tool)
    for face, cutter in pairs:
        section = BRepAlgoAPI_Section(face, cutter, False)
        section.ComputePCurveOn1(True)  # SplitShape needs the edges drawn on the face
        section.Approximation(True)
        section.Build()
        if not section.IsDone():
            raise Refused("BRepAlgoAPI_Section could not cross a face of the tool with its cut")
        for edge in found(section.Shape(), TopAbs_EDGE):
            splitting.Add(TopoDS.Edge(edge), TopoDS.Face(face))
    splitting.Build()
    if not splitting.IsDone():
        raise Refused("BRepFeat_SplitShape could not split the tool's faces along their contact")
    return splitting.Shape()


def split_shell(tool, cuts):
    """The shell of `tool` split by the faces `cuts`."""
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(shapes(*found(tool, TopAbs_SHELL)))
    splitter.SetTools(shapes(*cuts))
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not split the tool along its contact")
    return splitter.Shape()


def split_solid(tool, cuts):
    """`tool` with its faces split by the faces `cuts`, still one solid."""
    shells = found(split_shell(tool, cuts), TopAbs_SHELL)
    made = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells[0]))
    if len(shells) != 1 or not made.IsDone():
        raise Refused("the tool's split faces did not close into a solid")
    return made.Solid()


def leads(face, velocity):
    """+1 if `face` leads the motion all over (its outward normal at an acute angle to
    `velocity(point)`), −1 if it trails all over, 0 if the motion keeps it in itself; sampled at a
    grid of points of the face."""
    face = TopoDS.Face(face)
    props = BRepGProp_Face(face)
    u0, u1, v0, v1 = BRepTools.UVBounds_s(face)
    signs = set()
    for i in range(SAMPLES):
        for j in range(SAMPLES):
            u = u0 + (u1 - u0) * (i + 0.5) / SAMPLES
            v = v0 + (v1 - v0) * (j + 0.5) / SAMPLES
            if BRepClass_FaceClassifier(face, gp_Pnt2d(u, v), 1e-9).State() != TopAbs_IN:
                continue
            p, n = gp_Pnt(), gp_Vec()
            props.Normal(u, v, p, n)  # outward: BRepGProp_Face heeds the face's orientation
            w = velocity(p)
            if n.Magnitude() == 0 or w.Magnitude() == 0:
                continue
            c = n.Normalized().Dot(w.Normalized())
            if abs(c) >= SQUARE:
                signs.add(1 if c > 0 else -1)
    assert len(signs) <= 1, "a face is split short of its contact: it both leads and trails"
    return signs.pop() if signs else 0


def leading(tool, cuts, velocity):
    """The faces of `tool` that lead the motion, split along their contact by `cuts`."""
    return [f for f in split(tool, cuts) if leads(f, velocity) > 0]


def slid(tool, vector, cuts=()):
    """The tool where it starts and the solids its leading faces sweep, as it slides by `vector`
    (x, y, z); `cuts` split its faces along their contact."""
    slide = gp_Vec(*vector)
    swept = []
    for face in leading(tool, cuts, lambda p: slide):
        prism = BRepPrimAPI_MakePrism(face, slide)
        if not prism.IsDone():
            raise Refused("BRepPrimAPI_MakePrism could not slide a face of the tool")
        swept.append(prism.Shape())
    return [tool, *swept]


def turned(tool, axis, degrees, cuts=(), span=180):
    """The tool where it starts and the solids its leading faces sweep, as it turns by `degrees`
    about `axis` (a gp_Ax1), right-handed; `cuts` split its faces along their contact.

    A turn is swept at most `span` degrees at a time, each span from where the last stopped:
    turned whole, a face sweeps a closed ring that may lie on another face's, and OCCT's
    Booleans do not always unite rings that coincide. The tool where each span starts lies in
    what the span before it swept."""
    origin, direction = axis.Location(), gp_Vec(axis.Direction())
    faces = leading(tool, cuts, lambda p: direction.Crossed(gp_Vec(origin, p)))
    spans = math.ceil(abs(degrees) / span - 1e-9)
    step = degrees / spans
    swept = [tool]
    for k in range(spans):
        for face in faces:
            revol = BRepPrimAPI_MakeRevol(turn(face, axis, k * step), axis, math.radians(step))
            if not revol.IsDone():
                raise Refused("BRepPrimAPI_MakeRevol could not turn a face of the tool")
            swept.append(outward(flattened(revol.Shape(), axis)))
    return swept


def outward(solid):
    """`solid`, its faces turned outward if a sweep left them facing in."""
    props = GProp_GProps()
    BRepGProp.VolumeProperties_s(solid, props)
    return solid.Reversed() if props.Mass() < 0 else solid


def flattened(solid, axis):
    """`solid` with each face that turning a curve square to `axis` made rebuilt flat.

    A curve that lies in a plane square to the axis turns within that plane, and MakeRevol
    makes of it a surface of revolution all of whose normals are along the axis: flat, but
    not a plane, and OCCT's Booleans do not find it lying in the plane of a flat face beside
    it. Each such face is rebuilt on the plane it lies in, from the same edges."""
    d = axis.Direction()
    reshape = BRepTools_ReShape()
    changed = False
    for face in found(solid, TopAbs_FACE):
        face = TopoDS.Face(face)
        if BRepAdaptor_Surface(face).GetType() != GeomAbs_SurfaceOfRevolution:
            continue
        heights = [gp_Vec(axis.Location(), BRep_Tool.Pnt_s(TopoDS.Vertex(v))).Dot(gp_Vec(d))
                   for v in found(face, TopAbs_VERTEX)]
        props = BRepGProp_Face(face)
        u0, u1, v0, v1 = BRepTools.UVBounds_s(face)
        p, n = gp_Pnt(), gp_Vec()
        props.Normal((u0 + u1) / 2, (v0 + v1) / 2, p, n)
        level = gp_Vec(axis.Location(), p).Dot(gp_Vec(d))
        if max(abs(h - level) for h in heights) > 1e-9 or n.Magnitude() == 0 \
                or n.Normalized().Crossed(gp_Vec(d)).Magnitude() > 1e-9:
            continue
        # the plane through the face, its normal the face's own, outward
        normal = d if n.Dot(gp_Vec(d)) > 0 else d.Reversed()
        wires = [TopoDS.Wire(w) for w in found(face, TopAbs_WIRE)]
        outer = BRepTools.OuterWire_s(face)
        flat = BRepBuilderAPI_MakeFace(gp_Pln(p, normal), outer, True)
        for wire in wires:
            if not wire.IsSame(outer):
                flat.Add(wire)
        if not flat.IsDone():
            raise Refused("a flat face that MakeRevol made could not be rebuilt on its plane")
        reshape.Replace(face, flat.Face())
        changed = True
    return reshape.Apply(solid) if changed else solid


def risen(lead, growth, degrees):
    """How far a thread's path has risen at a turn of `degrees`: `lead`·s + `growth`·s², s the
    number of whole turns."""
    s = degrees / 360
    return lead * s + growth * s * s


def threaded_motion(lead, growth, start, degrees):
    """The motion that carries a tool from a turn of `start` to one of `degrees` along a thread
    (see `risen`): a turn about the z axis, and a rise along it."""
    trsf = gp_Trsf()
    trsf.SetRotation(gp.OZ_s(), math.radians(degrees - start))
    rise = gp_Trsf()
    rise.SetTranslation(gp_Vec(0, 0, risen(lead, growth, degrees) - risen(lead, growth, start)))
    return rise.Multiplied(trsf)


def thread_path(lead, growth, start, stop, radius=1):
    """The path of a point at `radius` from the z axis, at a turn of `start` to `stop` degrees, as
    a wire: OCCT draws it as a curve in the (angle, height) parameters of a cylinder, a straight
    line for a screw and a parabola where the lead grows, each a Bézier curve of degree 2."""
    cylinder = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), radius)
    a0, a1 = math.radians(start), math.radians(stop)
    h0, h1 = risen(lead, growth, start), risen(lead, growth, stop)
    slope = (lead + 2 * growth * start / 360) / (2 * math.pi)  # rise per radian at the start
    poles = Array1_gp_Pnt2d(1, 3)
    for i, pole in enumerate([(a0, h0), ((a0 + a1) / 2, h0 + slope * (a1 - a0) / 2), (a1, h1)]):
        poles.SetValue(i + 1, gp_Pnt2d(*pole))
    edge = BRepBuilderAPI_MakeEdge(Geom2d_BezierCurve(poles), cylinder).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)  # the path in space, from the curve on the cylinder
    return BRepBuilderAPI_MakeWire(edge).Wire()


def first_vertex(wire):
    return TopExp.FirstVertex_s(TopoDS.Edge(found(wire, TopAbs_EDGE)[0]))


def rim_sheets(spine, faces):
    """The sheets the rim of the surface `faces` (the edges only one of them has) sweeps as it
    is carried by the frame riding `spine` whose binormal stays the z axis
    (BRepOffsetAPI_MakePipeShell, GeomFill_ConstantBiNormal): the frame turns about z as the
    spine's tangent turns, and rises as the spine rises. Along a thread's path about the z axis,
    that is the motion that carries the path along itself."""
    rims = ShapeAnalysis_FreeBounds(compound(faces)).GetClosedWires()
    sheets = []
    for rim in found(rims, TopAbs_WIRE):
        pipe = BRepOffsetAPI_MakePipeShell(spine)
        pipe.SetMode(gp.DZ_s())
        # the sheets are B-splines the sweep fits: ask for them within 1e-8, not the 1e-4 it
        # settles for by default
        pipe.SetTolerance(1e-8, 1e-8, 1e-6)
        # the rim is the tool's where the motion starts: set it at the spine's first vertex, not
        # where the rim's plane would meet the spine
        pipe.Add(TopoDS.Wire(rim), first_vertex(spine), False, False)
        pipe.Build()
        if not pipe.IsDone():
            raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the leading faces' rim")
        sheets.append(pipe.Shape())
    return sheets


def sewn(pieces):
    """The solid the sheets and faces `pieces` close, sewn."""
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for piece in pieces:
        sewing.Add(piece)
    sewing.Perform()
    solids = []
    for shell in found(sewing.SewedShape(), TopAbs_SHELL):
        made = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shell))
        if not made.IsDone() or not BRep_Tool.IsClosed_s(shell):
            raise Refused("the leading faces' swept rim and the faces at each end did not sew "
                          "into a solid")
        solid = made.Solid()
        BRepLib.OrientClosedSolid_s(solid)
        solids.append(outward(solid))
    if not solids:
        raise Refused("the leading faces' swept rim and the faces at each end sewed into nothing")
    return solids[0] if len(solids) == 1 else compound(solids)


def piped(spine, faces, ends):
    """The solid the surface `faces` sweeps as the frame riding `spine` carries it (see
    `rim_sheets`): the sheets its rim sweeps, closed by the faces where they start and `ends`,
    the same faces where they stop, sewn into one solid, with no sheet of its own between two
    neighbouring faces."""
    return sewn([*rim_sheets(spine, faces), *faces, *ends])


def compound(shapes_):
    built, builder = TopoDS_Compound(), BRep_Builder()
    builder.MakeCompound(built)
    for item in shapes_:
        builder.Add(built, item)
    return built


def threaded(tool, lead, start, stop, cuts=(), growth=0, span=None, whole=False):
    """The tool where it starts and what its leading faces sweep, as it is carried along a
    thread about the z axis, from a turn of `start` degrees (where `tool` stands) to one of
    `stop`: turning right-handed about the axis and rising `lead`·s + `growth`·s² at s whole
    turns, rigidly (a screw where `growth` is 0). `cuts` split its faces along their contact.
    Which faces lead is judged where the motion starts; a case whose faces lead differently later
    says so.

    The motion is swept a span of at most `span` degrees (SPAN unless given) at a time, each
    span from where the last stopped: OCCT's Booleans find no intersection with a sheet wound
    round more than once, and a groove wider than the lead overlaps itself within a turn. The
    tool where each span starts lies in what the span before it swept. Each leading face is
    swept on its own, a solid for each face and span; with `whole`, the leading faces of a span
    are swept together into one solid (`piped`), which a case may ask for only where what the
    faces sweep cannot overlap: where every circle about the axis meets the tool in one arc, so
    that no point is entered twice in a pass."""
    slope = (lead + 2 * growth * start / 360) / (2 * math.pi)
    faces = leading(tool, cuts, lambda p: gp_Vec(-p.Y(), p.X(), slope))
    spans = math.ceil((stop - start) / (span or SPAN) - 1e-9)
    swept = [tool]
    for k in range(spans):
        a = start + (stop - start) * k / spans
        b = start + (stop - start) * (k + 1) / spans
        path = thread_path(lead, growth, a, b)
        # moved together, so that the faces keep the edges they share
        at_a, at_b = (found(moved(compound(faces), threaded_motion(lead, growth, start, d)),
                            TopAbs_FACE) for d in (a, b))
        if whole:
            swept.append(piped(path, at_a, at_b))
        else:
            swept += [piped(path, [f], [g]) for f, g in zip(at_a, at_b)]
    return swept


def screwed(tool, lead, start, stop, cuts=(), span=None, whole=False):
    """`threaded` along a screw: the tool turns about the z axis and rises `lead` a turn."""
    return threaded(tool, lead, start, stop, cuts, span=span, whole=whole)


def flat_contacts_turned(tool, axis):
    """Where a turn about `axis` (a gp_Ax1) runs along each flat face of `tool`: a point p of a
    plane with normal n moves square to n where n·(d × (p − o)) = 0, that is where
    (n × d)·(p − o) = 0, d the axis's direction and o a point of it. That is the plane through
    the axis and n; it cuts the face along its contact, if anywhere. (A face square to the axis
    turns in its own plane, and has none.)"""
    d, o = axis.Direction(), axis.Location()
    cuts = []
    for face in found(tool, TopAbs_FACE):
        surface = BRepAdaptor_Surface(TopoDS.Face(face))
        if surface.GetType() != GeomAbs_Plane:
            continue
        m = gp_Vec(surface.Plane().Axis().Direction()).Crossed(gp_Vec(d))
        if m.Magnitude() > 1e-9:
            cuts.append(plane(o.Coord(), m.Coord()))
    return cuts


def flat_contacts_screwed(tool, lead, reach=REACH):
    """Where a screw about the z axis that rises `lead` a turn runs along each flat face of
    `tool`: a point p moves with velocity ẑ × p + kẑ, k = lead/2π, square to a face's normal n
    where (n × ẑ)·p + k n_z = 0. That is a plane parallel to the axis, reaching `reach` either
    way; it cuts the face along its contact, if anywhere, and no other face. (A level face rises
    or sinks all over, and has none.)"""
    k = lead / (2 * math.pi)
    cuts = []
    for face in found(tool, TopAbs_FACE):
        surface = BRepAdaptor_Surface(TopoDS.Face(face))
        if surface.GetType() != GeomAbs_Plane:
            continue
        n = gp_Vec(surface.Plane().Axis().Direction())
        m = n.Crossed(gp_Vec(0, 0, 1))
        if m.Magnitude() > 1e-9:
            # the point of that plane nearest the axis, at height 0
            at = m.Multiplied(-k * n.Z() / m.SquareMagnitude())
            cuts.append((face, plane(at.Coord(), m.Coord(), reach)))
    return cuts


def united(pieces, what="the tool and what its faces sweep", glue=False):
    """The union of `pieces`, refusing, naming `what`, if OCCT declines. With `glue`, the pieces
    only touch, along faces they share, and never overlap (BOPAlgo_GlueShift): a case may ask
    for it only where that is so, as for the faces of a convex solid slid, each point entered
    once."""
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(shapes(pieces[0]))
    fuse.SetTools(shapes(*pieces[1:]))
    if glue:
        fuse.SetGlue(BOPAlgo_GlueEnum.BOPAlgo_GlueShift)
    fuse.Build()
    if not fuse.IsDone():
        raise Refused(f"BRepAlgoAPI_Fuse declined {what}")
    return fuse.Shape()


def cut(block, pieces, what="the tool and what its faces sweep"):
    """`block` less `pieces`, cut one after another, as holes are; refusing, naming `what`, if
    OCCT declines."""
    for piece in pieces:
        cutting = BRepAlgoAPI_Cut(block, piece)
        if not cutting.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined {what}")
        block = cutting.Shape()
    return block


def ring_sector(inner, outer, low, high, start, stop):
    """The block of the swung cases: everything between `inner` and `outer` from the z axis,
    between heights `low` and `high`, and between polar angles `start` and `stop` (degrees, from
    +x towards +y). It is the rectangle of those radii and heights in the half-plane at angle
    `start`, turned about the z axis to `stop`."""
    a = math.radians(start)
    c, s = math.cos(a), math.sin(a)
    corners = [gp_Pnt(r * c, r * s, z) for r, z in
               ((inner, low), (outer, low), (outer, high), (inner, high))]
    section = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*corners, True).Wire()).Face()
    revol = BRepPrimAPI_MakeRevol(section, gp.OZ_s(), math.radians(stop - start))
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol could not turn the block's section")
    return revol.Shape()


def unified(shape):
    """`shape` with the faces that lie on one surface merged into one face
    (ShapeUpgrade_UnifySameDomain): a sweep built in spans and pieces leaves its walls in
    pieces, and BRepMesh does not always mesh the seams between them alike from both sides."""
    unify = ShapeUpgrade_UnifySameDomain(shape, True, True, True)
    unify.Build()
    return unify.Shape()

