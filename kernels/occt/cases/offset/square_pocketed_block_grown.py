"""A block with a square pocket grown by 1. The block from (0, 0, 0) to (20, 20, 6) with a square
pocket cut down into its top, the square from (6, 6) to (14, 14), its corners square, from the top
down to a flat floor at z = 3, grown by 1: every point within 1 of it."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from offset import less, offset

BLOCK = ((0, 0, 0), (20, 20, 6))
POCKET = ((6, 6, 3), (14, 14, 7))  # the pocket's cutter, running 1 above the block's top
GROWTH = 1


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape()
    cutter = BRepPrimAPI_MakeBox(gp_Pnt(*POCKET[0]), gp_Pnt(*POCKET[1])).Shape()
    return offset(less(block, cutter, "the pocket"), GROWTH)
