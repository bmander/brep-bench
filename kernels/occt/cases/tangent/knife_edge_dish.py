"""A cylinder with a knife-edged dish. A cylinder of radius 5 on the z axis, from z = 0 to z = 10.
Cut from it a ball of radius 5 centred at (0, 0, 10), the middle of its top face. The ball's
equator is the rim of the top face, so the whole top face goes, and the dish meets the wall
tangentially along that circle: a knife edge."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 5
HEIGHT = 10


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, HEIGHT), RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(cylinder, ball)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the dish")
    return cut.Shape()
