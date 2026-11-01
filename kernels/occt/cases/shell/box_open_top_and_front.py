"""A box shelled open at two faces that share an edge. The box from the origin to (10, 8, 6). Shell it
inward to a wall 1 thick, open at two faces that share an edge, its top, z = 6, and its front, y =
0. The outside of the wall is the box's other four faces, unmoved, and the inside is each of them
moved 1 inward along its normal, onto the surface offset 1 from it. The moved faces are extended or
trimmed where they meet one another, and run on until they meet the surface of an opening, which is
left as a rim between the outside and the inside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from shell import face_through, shell

SIZE = (10, 8, 6)
THICKNESS = 1


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    # the top, z = 6, and the front, y = 0, which share the edge y = 0, z = 6
    return shell(box, [face_through(box, (5, 4, 6)), face_through(box, (5, 0, 3))], THICKNESS)
