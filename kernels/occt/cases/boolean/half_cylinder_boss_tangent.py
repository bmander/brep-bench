"""A cube with a half-round end. Take a cube of side 10 between the origin and (10, 10, 10).
Unite it with a cylinder of radius 5 and height 10 that stands on the xy plane, its axis at
x = 10, y = 5. Half of the cylinder lies inside the cube. Its surface touches the cube's faces
y = 0 and y = 10 exactly along the cube's edges at x = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

SIDE = 10
BOSS_RADIUS = 5
BOSS_HEIGHT = 10


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 5, 0), gp.DZ_s()),
                                    BOSS_RADIUS, BOSS_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    return fuse.Shape()
