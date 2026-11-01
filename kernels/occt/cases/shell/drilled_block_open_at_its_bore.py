"""A drilled block shelled open at its bore. The block from the origin to (20, 20, 10) with a hole of
radius 3 bored right through it along z, its axis the line x = 10, y = 10. Shell it inward to a wall
1 thick, open at the bore, its cylindrical face of radius 3. The outside of the wall is the block's
six flat faces, unmoved, and the inside is each of them moved 1 inward along its normal, onto the
surface offset 1 from it. The moved faces are extended or trimmed where they meet one another, and
run on until they meet the surface of an opening, which is left as a rim between the outside and the
inside."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from shell import face_through, shell

SIZE = (20, 20, 10)
HOLE_RADIUS = 3
THICKNESS = 1
AXIS = (10, 10)


def drilled(x, y):
    """The block with the hole bored along z through (x, y), its cylinder running past both
    faces."""
    block = BRepPrimAPI_MakeBox(*SIZE).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, -1), gp_Dir(0, 0, 1)), HOLE_RADIUS,
                                    SIZE[2] + 2).Shape()
    cut = BRepAlgoAPI_Cut(block, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    return cut.Shape()


def build():
    solid = drilled(*AXIS)
    bore = face_through(solid, (AXIS[0], AXIS[1] - HOLE_RADIUS, SIZE[2] / 2))
    return shell(solid, [bore], THICKNESS)
