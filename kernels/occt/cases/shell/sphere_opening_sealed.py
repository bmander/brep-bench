"""A ball cut by a plane and shelled open there, the cavity never reaching it. The part of the ball of
radius 5 centred on the origin below the plane z = 4.5. Shell it inward to a wall 1 thick, open at
its flat face, z = 4.5. The outside of the wall is its sphere, unmoved, and the inside is the sphere
moved 1 inward along its normal, onto the surface offset 1 from it. The moved faces are extended or
trimmed where they meet one another, and run on until they meet the surface of an opening, which is
left as a rim between the outside and the inside."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused
from shell import face_through, shell

RADIUS = 5
THICKNESS = 1
HEIGHT = 4.5


def ball_below(height):
    """The ball less everything above the plane z = height."""
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, height), gp.DZ_s())).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, height + 1)).Solid()
    cut = BRepAlgoAPI_Cut(ball, above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()


def build():
    solid = ball_below(HEIGHT)
    return shell(solid, [face_through(solid, (0, 0, HEIGHT))], THICKNESS)
