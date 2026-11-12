"""What the OCCT implementations of the edit cases share: moving, offsetting or deleting a solid's
faces.

Moving faces is BRepOffset_MakeOffset with no offset for the solid as a whole and an offset for
each face to move (SetOffsetOnFace). Each moves along its outward normal, and the faces beside it
are extended or trimmed to meet it (joined by intersection, so no edge is rounded). A plane moved
along its normal is the plane translated, and a cylinder offset is the cylinder about the same
axis with its radius changed. Deleting faces is BRepAlgoAPI_Defeaturing: it removes them and heals
the solid by extending the faces round them, along their own surfaces, until they close the gap.
"""
import math

from OCP.BRep import BRep_Tool
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Defeaturing, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.BRepPrimAPI import (BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder,
                             BRepPrimAPI_MakePrism)
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

TOLERANCE = 1e-7


def deleted(solid, faces):
    """`solid` with `faces` deleted and the gap they leave closed by the faces round them."""
    defeaturing = BRepAlgoAPI_Defeaturing()
    defeaturing.SetShape(solid)
    for face in faces:
        defeaturing.AddFaceToRemove(face)
    defeaturing.SetRunParallel(False)
    defeaturing.Build()
    if not defeaturing.IsDone():
        raise Refused(f"BRepAlgoAPI_Defeaturing could not delete the {len(faces)} faces")
    return defeaturing.Shape()


def moved(solid, moves):
    """`solid` with each (face, distance) of `moves` moved that far along its outward normal, and
    the faces beside it extended or trimmed to meet it."""
    offset = BRepOffset_MakeOffset()
    offset.Initialize(solid, 0.0, TOLERANCE, BRepOffset_Skin, False, False,
                      GeomAbs_Intersection, False, False)
    for face, distance in moves:
        offset.SetOffsetOnFace(face, distance)
    offset.MakeOffsetShape()
    if not offset.IsDone():
        raise Refused(f"BRepOffset_MakeOffset could not move the faces ({offset.Error().name})")
    return offset.Shape()


def drilled(solid, x, y, radius, z0, z1):
    """`solid` drilled along z by a hole of `radius` about the vertical line through (x, y), the
    cutter running from `z0` to `z1`, past the faces it comes out through."""
    cutter = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z0), gp_Dir(0, 0, 1)),
                                      radius, z1 - z0).Shape()
    cut = BRepAlgoAPI_Cut(solid, cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the hole")
    return cut.Shape()


def box(lo, hi):
    """The box from the corner `lo` to the corner `hi`."""
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def straight_edges(solid, on):
    """The edges of `solid` both of whose ends satisfy `on`, a test of a point, each once."""
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        ends = TopExp.FirstVertex_s(edge), TopExp.LastVertex_s(edge)
        if all(on(BRep_Tool.Pnt_s(end)) for end in ends) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
        edges.Next()
    return found


def cylinder(base, radius, height):
    """The cylinder of `radius` about the vertical line through `base`, from `base` up `height`."""
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*base), gp_Dir(0, 0, 1)), radius, height).Shape()


def frustum(base, radius, top_radius, height):
    """The cone frustum about the vertical line through `base`, from `radius` there to
    `top_radius` at `height` above it."""
    axes = gp_Ax2(gp_Pnt(*base), gp_Dir(0, 0, 1))
    return BRepPrimAPI_MakeCone(axes, radius, top_radius, height).Shape()


def extruded(corners, vector):
    """The polygon through `corners`, extruded along `vector`."""
    polygon = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        polygon.Add(gp_Pnt(*corner))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(*vector)).Shape()


def united(a, b):
    """`a` and `b` united."""
    union = BRepAlgoAPI_Fuse(a, b)
    if not union.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not unite the two solids")
    return union.Shape()


def less(a, b):
    """`a` with `b` cut away."""
    cut = BRepAlgoAPI_Cut(a, b)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the solid")
    return cut.Shape()


def middle_at(point):
    """A test of an edge's middle point, for blend.edges_where: whether it is `point`."""
    return lambda middle: middle.Distance(gp_Pnt(*point)) < TOLERANCE


def on_circle(center, radius):
    """A test of a point, for blend.edges_where: whether it is on the horizontal circle of `radius`
    about `center`."""
    def test(q):
        from_axis = math.hypot(q.X() - center[0], q.Y() - center[1])
        return abs(q.Z() - center[2]) < TOLERANCE and abs(from_axis - radius) < TOLERANCE
    return test
