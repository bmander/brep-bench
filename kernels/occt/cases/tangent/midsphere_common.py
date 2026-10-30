"""A cube trimmed by the ball tangent to its edges. A cube of side 10 centred on the origin, from
(−5, −5, −5) to (5, 5, 5), and a ball centred on the origin of radius 5√2, so that the sphere
touches each of the cube's twelve edges at its midpoint. Make the solid common to both."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

HALF = 5
RADIUS = 5 * sqrt(2)  # from the centre to the middle of an edge


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-HALF, -HALF, -HALF), gp_Pnt(HALF, HALF, HALF)).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    common = BRepAlgoAPI_Common(cube, ball)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and its edge-tangent ball")
    return common.Shape()
