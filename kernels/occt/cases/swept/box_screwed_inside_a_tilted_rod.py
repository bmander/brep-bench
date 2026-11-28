"""A sealed helical tunnel in a tilted rod: a box screwed inside it. The rod and tunnel of
swept/box-screwed-inside-a-rod, turned rigidly about the origin so that the point that case puts
at (x, y, z) is here at x·u + y·v + z·w, where u = (2, −2, 1)/3, v = (2, 1, −2)/3 and
w = (1, 2, 2)/3.

So: a rod, the cylinder of radius 5 about the line through the origin along w, from the origin to
20w, with a sealed helical tunnel inside it: everything the cube of side 1 with 2.5 ≤ p·u ≤ 3.5,
−0.5 ≤ p·v ≤ 0.5 and 2 ≤ p·w ≤ 3 passes through as it turns right-handed about w (from u toward
v), rising 4 along w for every whole turn, from a turn of 0° to one of 720°."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Ax3, gp_Dir, gp_Pnt, gp_Trsf

from bench import other_case

U = (2 / 3, -2 / 3, 1 / 3)
W = (1 / 3, 2 / 3, 2 / 3)  # the rod's axis; v = w × u


def build():
    # OCCT's sweeps of the box's faces along a screw (sweep.py's `screwed`) follow a screw about
    # the z axis, so the rod and its tunnel are built in the frame whose z axis is the rod's,
    # as swept/box-screwed-inside-a-rod builds them, and placed by the rigid motion that takes
    # that frame to u, v, w.
    upright = other_case("swept/box-screwed-inside-a-rod").build()
    placing = gp_Trsf()
    placing.SetDisplacement(gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)),
                            gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(*W), gp_Dir(*U)))
    return BRepBuilderAPI_Transform(upright, placing, True).Shape()
