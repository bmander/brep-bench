"""A half cylinder with every edge rounded. The half of the cylinder of radius 5 about the z axis
from z = 0 to z = 10 where y ≥ 0, with all eight of its edges rounded to radius 1: the two
half-circle rims, the two straight edges where the round face meets the flat one, and the four
where the flat face meets the ends. Three rounds meet at each of the four corners."""
import math

from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 10
ROUND = 1


def build():
    # A cylinder of a half turn, from +x round through +y to −x: the side y ≥ 0.
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0))
    bar = BRepPrimAPI_MakeCylinder(frame, RADIUS, HEIGHT, math.pi).Shape()
    fillet = BRepFilletAPI_MakeFillet(bar)
    # Every edge of the bar; MakeFillet takes an edge it already holds only once.
    edges = TopExp_Explorer(bar, TopAbs_EDGE)
    while edges.More():
        fillet.Add(ROUND, TopoDS.Edge(edges.Current()))
        edges.Next()
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the half cylinder's edges")
    return fillet.Shape()
