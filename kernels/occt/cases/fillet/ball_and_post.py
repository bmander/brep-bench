"""A post standing out of a ball, rounded where it leaves. A ball of radius 10 centred on the
origin, and a post: a cylinder of radius 4 on the z axis from z = 0 to z = 15, joined to the ball
into one solid. The post leaves the ball on the circle of radius 4 at z = √84.

Round that circular edge to radius 2, adding material. The round is a torus about the z axis
tangent to the ball and to the post: its centre circle has radius 6 (4 + 2) and lies 12 (10 + 2)
from the origin, at z = 6√3. It touches the post on the circle of radius 4 at z = 6√3 and the
ball on the circle of radius 5 at z = 5√3, and covers the ball between there and the post."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS

from bench import Refused

BALL_RADIUS = 10
POST_RADIUS, POST_HEIGHT = 4, 15
RADIUS = 2
TOLERANCE = 1e-7


def build():
    ball = BRepPrimAPI_MakeSphere(BALL_RADIUS).Shape()
    post = BRepPrimAPI_MakeCylinder(POST_RADIUS, POST_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(ball, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    part = fuse.Shape()

    fillet = BRepFilletAPI_MakeFillet(part)
    # Where the post leaves the ball: the circle of the post's radius below its top.
    edges = TopExp_Explorer(part, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle:
            circle = curve.Circle()
            if (abs(circle.Radius() - POST_RADIUS) < TOLERANCE
                    and circle.Location().Z() < POST_HEIGHT - TOLERANCE):
                fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} circles where the post leaves"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the post's root")
    return fillet.Shape()
