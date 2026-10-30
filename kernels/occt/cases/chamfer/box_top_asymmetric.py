"""A box with its top edges chamfered unequally. A box from the origin to (20, 10, 6). Chamfer
the four edges around its top face z = 6, each with distance 1 measured across the top face and
distance 2 measured down the side face: the chamfer of the edge at y = 10, z = 6 is the plane
through the lines y = 9, z = 6 and y = 10, z = 4, and the other three likewise.

The solid is the box cut by those four planes. Its top face is the rectangle from (1, 1, 6) to
(19, 9, 6); its four vertical edges end at z = 4, and the chamfers of two neighbouring edges meet
along a slanting line from the box's corner at z = 4 to the top face's corner, from (20, 10, 4)
to (19, 9, 6) and so on. Each chamfer face is a trapezoid; four faces meet at each of the points
such as (20, 10, 4)."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

LENGTH, WIDTH, HEIGHT = 20, 10, 6
ACROSS_TOP, DOWN_SIDE = 1, 2
TOLERANCE = 1e-7


def top_face(box):
    """The box's face z = 6: the plane whose origin is at that height."""
    faces = TopExp_Explorer(box, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        surface = BRepAdaptor_Surface(face)
        plane = surface.Plane()
        if (abs(abs(plane.Axis().Direction().Z()) - 1) < TOLERANCE
                and abs(plane.Location().Z() - HEIGHT) < TOLERANCE):
            return face
        faces.Next()
    raise AssertionError("no face at z = 6")


def build():
    box = BRepPrimAPI_MakeBox(LENGTH, WIDTH, HEIGHT).Shape()
    top = top_face(box)
    chamfer = BRepFilletAPI_MakeChamfer(box)
    # The top face's four edges, each with its first distance measured on the top face.
    edges = TopExp_Explorer(top, TopAbs_EDGE)
    while edges.More():
        chamfer.Add(ACROSS_TOP, DOWN_SIDE, TopoDS.Edge(edges.Current()), top)
        edges.Next()
    assert chamfer.NbContours() >= 1, "no edges round the top"
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edges round the top")
    return chamfer.Shape()
