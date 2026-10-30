"""A cube and its inscribed ball, their common part. A cube of side 10 between the origin and
(10, 10, 10), and the ball of radius 5 centred at the cube's centre, (5, 5, 5), touching each face
at its middle. Make the solid common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 10
RADIUS = 5


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(SIDE / 2, SIDE / 2, SIDE / 2), RADIUS).Shape()
    common = BRepAlgoAPI_Common(cube, ball)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and its inscribed ball")
    return common.Shape()
