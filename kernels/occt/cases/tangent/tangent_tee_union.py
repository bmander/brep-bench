"""A tee whose branch touches the main pipe's crown. Two solid cylinders crossing at right angles.
The first, the main, of radius 5, lies along the x axis from x = −15 to x = 15. The second, the
branch, of radius 3, runs parallel to the y axis from y = −15 to y = 15, its axis raised to the
line x = 0, z = 2. Unite them. The tops of both are at z = 5, where the surfaces are tangent."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

MAIN_RADIUS = 5
BRANCH_RADIUS = 3
BRANCH_Z = 2  # the branch's axis is the line x = 0, z = 2
LENGTH = 30


def build():
    main = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-LENGTH / 2, 0, 0), gp.DX_s()),
                                    MAIN_RADIUS, LENGTH).Shape()
    branch = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -LENGTH / 2, BRANCH_Z), gp.DY_s()),
                                      BRANCH_RADIUS, LENGTH).Shape()
    fuse = BRepAlgoAPI_Fuse(main, branch)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the tee")
    return fuse.Shape()
