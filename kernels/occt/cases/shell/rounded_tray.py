"""A plate with rounded corners shelled thinner than its rounds. The box from the origin to (20, 14,
10) with its four vertical edges rounded to radius 2: a plate whose outline is the 20 × 14 rectangle
with its corners rounded to radius 2, 10 high. Shell it inward to a wall 1 thick, open at its top
face, z = 10. The outside of the wall is the plate's other faces, its rounds among them, unmoved,
and the inside is each of them moved 1 inward along its normal, onto the surface offset 1 from it.
The moved faces are extended or trimmed where they meet one another, and run on until they meet the
surface of an opening, which is left as a rim between the outside and the inside."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused
from shell import face_through, shell

SIZE = (20, 14, 10)
RADIUS = 2
THICKNESS = 1


def plate():
    """The box with its four vertical edges rounded: the edges whose ends differ only in z."""
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    fillet = BRepFilletAPI_MakeFillet(box)
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if vertical(edge):
            fillet.Add(RADIUS, edge)
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vertical edges")
    return fillet.Shape()


def vertical(edge):
    curve = BRepAdaptor_Curve(edge)
    a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
    return abs(a.X() - b.X()) < 1e-9 and abs(a.Y() - b.Y()) < 1e-9


def build():
    solid = plate()
    return shell(solid, [face_through(solid, (10, 7, SIZE[2]))], THICKNESS)
