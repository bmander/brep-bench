"""What the OCCT implementations of the offset cases share: growing or shrinking a solid by a
distance, with BRepOffsetAPI_MakeOffsetShape.

A positive distance grows the solid and a negative one shrinks it. Joined by arcs (GeomAbs_Arc),
the faces moved apart are joined by rounds and the faces moved through each other are trimmed, so
the result is every point within the distance of the solid, or at least that far inside it. Joined
by intersection (GeomAbs_Intersection), the moved faces are extended until they meet, and every
edge stays sharp.
"""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepOffset import BRepOffset_Skin
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere
from OCP.GeomAbs import GeomAbs_Arc
from OCP.TopAbs import TopAbs_COMPOUND, TopAbs_SOLID
from OCP.TopExp import TopExp_Explorer
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

TOLERANCE = 1e-7


def offset(solid, distance, join=GeomAbs_Arc):
    """The solid grown by `distance` (shrunk where it is negative); Refused if OCCT declines."""
    # A Boolean's result is a compound holding the solid; given the compound, the offset answers
    # a shell, not a solid, so it is given the solid itself.
    solids = TopExp_Explorer(solid, TopAbs_SOLID)
    if solid.ShapeType() == TopAbs_COMPOUND and solids.More():
        only = solids.Current()
        solids.Next()
        if not solids.More():
            solid = only
    maker = BRepOffsetAPI_MakeOffsetShape()
    # Skin mode offsets every face. Intersection = True intersects every moved face with every
    # other, not only with those beside it, so moved faces that cross faces across a groove or a
    # flange are trimmed too.
    maker.PerformByJoin(solid, distance, TOLERANCE, BRepOffset_Skin, True, False, join)
    if not maker.IsDone():
        raise Refused(f"BRepOffsetAPI_MakeOffsetShape declined to offset the solid by {distance}")
    if maker.Shape().IsNull():
        raise Refused(f"BRepOffsetAPI_MakeOffsetShape answered done but made no shape of the solid "
                      f"offset by {distance}")
    return maker.Shape()


def revolved(section, what):
    """The solid of revolution about the z axis whose half-section in (ρ, z) is `section`, a closed
    outline given in order as ('to', ρ, z), a corner to run straight to, or ('arc', ρ, z, radius,
    from, to), an arc about that center between those angles in degrees; Refused, naming `what`, if
    OCCT makes no solid of it."""
    from math import cos, radians, sin

    from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                    BRepBuilderAPI_MakeWire)
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
    from OCP.GC import GC_MakeArcOfCircle
    from OCP.gp import gp, gp_Pnt

    def at(rho, z):
        return gp_Pnt(rho, 0, z)

    def on(center, radius, angle):
        turn = radians(angle)
        return (center[0] + radius * cos(turn), center[1] + radius * sin(turn))

    wire = BRepBuilderAPI_MakeWire()
    here = None
    first = None
    for piece in section:
        if piece[0] == 'to':
            there = (piece[1], piece[2])
            if here is not None and there != here:
                wire.Add(BRepBuilderAPI_MakeEdge(at(*here), at(*there)).Edge())
        else:
            center, radius, start, end = (piece[1], piece[2]), piece[3], piece[4], piece[5]
            begin, middle, there = (on(center, radius, a) for a in (start, (start + end) / 2, end))
            if here is not None and max(abs(begin[0] - here[0]), abs(begin[1] - here[1])) > 1e-9:
                wire.Add(BRepBuilderAPI_MakeEdge(at(*here), at(*begin)).Edge())
            arc = GC_MakeArcOfCircle(at(*begin), at(*middle), at(*there)).Value()
            wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
            first = first or begin
        first = first or there
        here = there
    wire.Add(BRepBuilderAPI_MakeEdge(at(*here), at(*first)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused(f"BRepBuilderAPI_MakeFace made no face of {what}")
    solid = BRepPrimAPI_MakeRevol(face.Face(), gp.OZ_s())
    if not solid.IsDone():
        raise Refused(f"BRepPrimAPI_MakeRevol declined {what}")
    return solid.Shape()


def upright_cone(z, height):
    """The cone on the z axis with its base, of radius `height`/2, at `z` and its apex `height`
    above it."""
    axes = gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1))
    return BRepPrimAPI_MakeCone(axes, height / 2, 0, height).Shape()


def ball_tipped_cone():
    """The cone on the z axis from its base of radius 6 at z = 0 to its apex at z = 12, its tip
    rounded by a ball of radius 1 tangent to its side: the frustum up to the circle of tangency
    united with the ball about (0, 0, 12 − √5)."""
    root = math.sqrt(5)
    axes = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))
    frustum = BRepPrimAPI_MakeCone(axes, 6, 2 / root, 12 - 4 / root).Shape()
    tip = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 12 - root), 1).Shape()
    union = BRepAlgoAPI_Fuse(frustum, tip)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not unite the cone and its tip")
    return union.Shape()


def less(solid, tool, what):
    """`solid` with `tool` cut away; Refused, naming `what`, if the cut declines."""
    cut = BRepAlgoAPI_Cut(solid, tool)
    if not cut.IsDone():
        raise Refused(f"BRepAlgoAPI_Cut declined to cut {what}")
    return cut.Shape()
