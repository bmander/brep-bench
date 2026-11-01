"""A box shelled outward with its top open, its edges rounded. The box from the origin to (10, 8, 6),
shelled outward to a wall 1 thick, open at its top face, z = 6. The box's faces other than the top
become the inside of the wall, and the wall is every point outside the box within 1 of those faces,
up to the plane of the top, z = 6, which is the rim. So the outside of the wall is each face moved 1
outward, joined at the box's convex edges by a quarter cylinder of radius 1 about the edge, and at
the bottom corners by an eighth of a ball of radius 1 about the corner."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from shell import face_through, shell

SIZE = (10, 8, 6)
THICKNESS = 1


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    # outward, joined by arcs: a quarter cylinder about each convex edge, a ball's eighth at each
    # corner
    return shell(box, [face_through(box, (5, 4, 6))], THICKNESS, outward=True)
