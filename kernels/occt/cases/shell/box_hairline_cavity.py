"""A box shelled nearly to half its width, leaving a cavity 0.02 wide. The box from the origin to (10,
8, 6). Shell it inward to a wall 3.99 thick, open at its top face, z = 6. The outside of the wall is
the box's other five faces, unmoved, and the inside is each of them moved 3.99 inward along its
normal, onto the surface offset 3.99 from it. The moved faces are extended or trimmed where they
meet one another, and run on until they meet the surface of an opening, which is left as a rim
between the outside and the inside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from shell import face_through, shell

SIZE = (10, 8, 6)
THICKNESS = 3.99


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    return shell(box, [face_through(box, (5, 4, 6))], THICKNESS)
