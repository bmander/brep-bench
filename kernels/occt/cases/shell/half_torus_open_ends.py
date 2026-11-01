"""A half torus shelled open at both ends: a curved pipe. The half with y ≥ 0 of the solid torus about
the z axis, centred on the origin, whose tube has radius 2 about a middle circle of radius 6. It has
two flat faces, the discs of radius 2 about (6, 0, 0) and (−6, 0, 0) in the plane y = 0. Shell it
inward to a wall 0.5 thick, open at both flat faces. The outside of the wall is its torus face,
unmoved, and the inside is the torus face moved 0.5 inward along its normal, onto the surface offset
0.5 from it. The moved faces are extended or trimmed where they meet one another, and run on until
they meet the surface of an opening, which is left as a rim between the outside and the inside."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused
from shell import face_through, shell

MAJOR, MINOR = 6, 2
THICKNESS = 0.5


def build():
    torus = BRepPrimAPI_MakeTorus(MAJOR, MINOR).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DY_s())).Face()
    below = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, -1, 0)).Solid()
    cut = BRepAlgoAPI_Cut(torus, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    half = cut.Shape()
    ends = [face_through(half, (MAJOR, 0, 0)), face_through(half, (-MAJOR, 0, 0))]
    return shell(half, ends, THICKNESS)
