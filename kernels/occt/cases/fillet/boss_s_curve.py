"""A boss whose root and rim rounds meet. A square plate from (−20, −20, 0) to (20, 20, 4), and
a round boss on it: a cylinder of radius 5 on the z axis standing on the plate's top face, from
z = 4 to z = 8, joined to the plate into one solid.

Round both circular edges of the boss to radius 2: the rim of its top face at z = 8 (convex,
removing material) and its root, the circle where it meets the plate at z = 4 (concave, adding
material). Each round takes 2 of the boss's height of 4, so the boss's cylindrical side
vanishes entirely: the two rounds meet on the circle of radius 5 at z = 6, tangent to each other
there, and the boss's profile is an S. The rim's round is a torus whose centre circle has radius
3 at z = 6; the root's is a torus whose centre circle has radius 7 at z = 6, and it meets the
plate's top on the circle of radius 7. The boss's top face is the disc of radius 3 at z = 8."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

HALF_SIDE, THICKNESS = 20, 4
BOSS_RADIUS, BOSS_HEIGHT = 5, 4
RADIUS = 2
TOLERANCE = 1e-7


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_SIDE, -HALF_SIDE, 0),
                                gp_Pnt(HALF_SIDE, HALF_SIDE, THICKNESS)).Shape()
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, THICKNESS), gp.DZ_s()),
                                    BOSS_RADIUS, BOSS_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()

    fillet = BRepFilletAPI_MakeFillet(part)
    # The boss's two circles, at its root and its top; the plate's edges are all straight.
    edges = TopExp_Explorer(part, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        if (curve.GetType() == GeomAbs_CurveType.GeomAbs_Circle
                and abs(curve.Circle().Radius() - BOSS_RADIUS) < TOLERANCE):
            fillet.Add(RADIUS, edge)
        edges.Next()
    assert fillet.NbContours() == 2, f"{fillet.NbContours()} circles of radius {BOSS_RADIUS}"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the boss's root and rim")
    return fillet.Shape()
