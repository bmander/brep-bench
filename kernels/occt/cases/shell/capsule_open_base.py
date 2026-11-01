"""A capsule's end shelled open at its base, its walls meeting tangentially. A capsule's end: a
cylinder of radius 5 on the z axis from z = 0 to z = 10, capped by the half ball of radius 5 centred
on (0, 0, 10), so that its top is a hemisphere joined to the side tangentially along the circle z =
10. Shell it inward to a wall 1 thick, open at its base, the disc at z = 0. The outside of the wall
is its side and its hemisphere, unmoved, and the inside is each of them moved 1 inward along its
normal, onto the surface offset 1 from it. The moved faces are extended or trimmed where they meet
one another, and run on until they meet the surface of an opening, which is left as a rim between
the outside and the inside."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused
from shell import face_through, shell

RADIUS = 5
HEIGHT = 10
THICKNESS = 1


def capsule_end():
    """The cylinder capped by the half ball on its top; the union merges the two into one solid
    whose dome is the sphere's upper half."""
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, HEIGHT), RADIUS).Shape()
    fuse = BRepAlgoAPI_Fuse(cylinder, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cylinder and the ball")
    return fuse.Shape()


def build():
    solid = capsule_end()
    return shell(solid, [face_through(solid, (0, 0, 0))], THICKNESS)
