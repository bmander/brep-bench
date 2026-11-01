"""A box shelled outward with its top open, its edges kept sharp. The box from the origin to (10, 8,
6), shelled outward to a wall 1 thick, open at its top face, z = 6, with its edges kept sharp. The
box's faces other than the top become the inside of the wall; the outside is each of them moved 1
outward along its normal, the moved faces extended until they meet, and cut off by the plane of the
top, z = 6, which is the rim."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_Intersection

from shell import face_through, shell

SIZE = (10, 8, 6)
THICKNESS = 1


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    # outward, joined by intersection: the moved faces extended until they meet
    return shell(box, [face_through(box, (5, 4, 6))], THICKNESS, outward=True,
                 join=GeomAbs_Intersection)
