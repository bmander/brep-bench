"""A cone standing on its tip inside a block. A block from (−10, −10, 0) to (10, 10, 10), united
with a solid cone on the z axis whose apex is at the origin, the middle of the block's bottom
face, and whose flat base is the disc of radius 5 in the plane z = 15."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Pnt

from bench import Refused

HALF_WIDTH = 10
BLOCK_HEIGHT = 10
CONE_RADIUS = 5
CONE_HEIGHT = 15


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_WIDTH, -HALF_WIDTH, 0),
                                gp_Pnt(HALF_WIDTH, HALF_WIDTH, BLOCK_HEIGHT)).Shape()
    cone = BRepPrimAPI_MakeCone(0, CONE_RADIUS, CONE_HEIGHT).Shape()  # apex at the origin
    fuse = BRepAlgoAPI_Fuse(block, cone)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block and the cone")
    return fuse.Shape()
