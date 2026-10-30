"""A block with a spherical pocket. A block from (−10, −10, −10) to (10, 10, 0). Cut from it a
ball of radius 5 centred on the origin, leaving a hemispherical pocket in its top face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

BALL_RADIUS = 5


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, -10), gp_Pnt(10, 10, 0)).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), BALL_RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(block, ball)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ball")
    return cut.Shape()
