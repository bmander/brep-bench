"""A box shelled open at every face but one. The box from the origin to (10, 8, 6). Shell it inward to
a wall 1 thick, open at five of its faces, every face but its bottom, z = 0. The outside of the wall
is the bottom alone, unmoved, and the inside is the bottom moved 1 inward along its normal, onto the
surface offset 1 from it. The moved faces are extended or trimmed where they meet one another, and
run on until they meet the surface of an opening, which is left as a rim between the outside and the
inside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from shell import face_through, shell

SIZE = (10, 8, 6)
THICKNESS = 1
# a point in the middle of each face but the bottom
OPENINGS = [(5, 4, 6), (0, 4, 3), (10, 4, 3), (5, 0, 3), (5, 8, 3)]


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    return shell(box, [face_through(box, p) for p in OPENINGS], THICKNESS)
