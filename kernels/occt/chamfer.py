"""What the OCCT implementations of the two-distance chamfer cases on a ball's edges share: the
circular edge through a point, the faces on either side of it, and BRepFilletAPI_MakeChamfer's
two-distance chamfer, whose distances are chords from the edge in the plane square to it."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeVertex
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused

TOLERANCE = 1e-7


def edge_through(solid, point):
    """The one edge of `solid` passing through `point`."""
    vertex = BRepBuilderAPI_MakeVertex(gp_Pnt(*point)).Vertex()
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        distance = BRepExtrema_DistShapeShape(vertex, edge)
        if (distance.IsDone() and distance.Value() < TOLERANCE
                and not any(edge.IsSame(e) for e in found)):
            found.append(edge)
        edges.Next()
    assert len(found) == 1, f"{len(found)} edges through {point}"
    return found[0]


def faces_of(solid, edge):
    """The faces of `solid` bounded by `edge`."""
    found = []
    faces = TopExp_Explorer(solid, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        edges = TopExp_Explorer(face, TopAbs_EDGE)
        while edges.More():
            if edges.Current().IsSame(edge):
                found.append(face)
                break
            edges.Next()
        faces.Next()
    return found


def is_ball_about(face, centre):
    """Whether `face` lies on a sphere centred at `centre`."""
    surface = BRepAdaptor_Surface(face)
    if surface.GetType() != GeomAbs_SurfaceType.GeomAbs_Sphere:
        return False
    return surface.Sphere().Location().Distance(gp_Pnt(*centre)) < TOLERANCE


def chamfered(solid, point, first, d_first, d_other, what):
    """`solid` with the edge through `point` chamfered `d_first` on the face for which `first`
    holds and `d_other` on the other; Refused, naming `what`, if OCCT declines."""
    edge = edge_through(solid, point)
    faces = [f for f in faces_of(solid, edge) if first(f)]
    assert len(faces) == 1, f"{len(faces)} faces of the edge through {point} hold the test"
    chamfer = BRepFilletAPI_MakeChamfer(solid)
    chamfer.Add(d_first, d_other, edge, faces[0])
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused(f"BRepFilletAPI_MakeChamfer could not chamfer {what}")
    return chamfer.Shape()
