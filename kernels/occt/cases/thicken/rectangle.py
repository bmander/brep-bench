"""A rectangular sheet thickened by 2. The rectangle in the xy plane from (0, 0, 0) to (10, 8, 0),
a single flat face, thickened by 2 to its +z side."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.gp import gp_Pln

from thicken import thicken

SIZE = (10, 8)
THICKNESS = 2


def build():
    # the plane z = 0, its normal +z, bounded in its own (u, v) = (x, y)
    sheet = BRepBuilderAPI_MakeFace(gp_Pln(), 0, SIZE[0], 0, SIZE[1]).Face()
    return thicken(sheet, THICKNESS)
