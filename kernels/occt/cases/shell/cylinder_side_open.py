"""A cylinder shelled open at its side: two separate discs. A cylinder of radius 5 on the z axis, from
z = 0 to z = 10. Shell it inward to a wall 1 thick, open at its curved side. The outside of the wall
is its two end faces, unmoved, and the inside is each of them moved 1 inward along its normal, onto
the surface offset 1 from it. The moved faces are extended or trimmed where they meet one another,
and run on until they meet the surface of an opening, which is left as a rim between the outside and
the inside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from shell import face_through, shell

RADIUS = 5
HEIGHT = 10
THICKNESS = 1


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    # the curved side, through a point off its seam
    return shell(cylinder, [face_through(cylinder, (0, RADIUS, HEIGHT / 2))], THICKNESS)
