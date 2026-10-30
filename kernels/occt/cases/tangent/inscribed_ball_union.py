"""A cube united with its inscribed ball. A cube of side 10 between the origin and (10, 10, 10),
united with the ball of radius 5 centred at the cube's centre, (5, 5, 5). The ball is inscribed:
it touches each of the cube's six faces at a single point, the middle of the face, and lies
everywhere else strictly inside the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 10
RADIUS = 5


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(SIDE / 2, SIDE / 2, SIDE / 2), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and its inscribed ball")
    return fuse.Shape()
