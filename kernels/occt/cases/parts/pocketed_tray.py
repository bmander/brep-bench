"""A pocketed tray with four blind-bored standoffs. An open tray lying on its back, its opening
facing −y. It is made in three steps.

1. A block from (−36, 0, −24) to (36, 16, 24): 72 in x, 16 in y and 48 in z.
2. A pocket taken out of it: the box from (−33, 0, −21) to (33, 13, 21). This leaves walls 3
   thick all round, and a floor 3 thick from y = 13 to y = 16. The pocket opens through the
   block's face at y = 0.
3. Four standoffs joined to the tray, standing on the floor inside the pocket. Each is a tube
   along y, outer radius 5 and inner radius 2, from y = 5 to y = 13: it stands 8 tall on the
   floor's inside face (y = 13), toward the opening. Their axes are the lines parallel to y
   through (x, z) = (26, 14), (−26, 14), (−26, −14) and (26, −14). Each tube's hole is blind: it
   runs from y = 5 down to the floor and stops there, and the floor under it is left whole.
   Each standoff clears the pocket's walls by 2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

BLOCK = ((-36, 0, -24), (36, 16, 24))
POCKET = ((-33, 0, -21), (33, 13, 21))
OUTER, INNER = 5, 2               # the standoffs' radii
STANDOFF_Y0, STANDOFF_Y1 = 5, 13
AXES = [(26, 14), (-26, 14), (-26, -14), (26, -14)]  # (x, z)


def box(corners):
    (x0, y0, z0), (x1, y1, z1) = corners
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def build():
    cut = BRepAlgoAPI_Cut(box(BLOCK), box(POCKET))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    tray = cut.Shape()
    for x, z in AXES:
        # A tube along +y from y = 5 to y = 13: a cylinder with its core taken out.
        axis = gp_Ax2(gp_Pnt(x, STANDOFF_Y0, z), gp.DY_s())
        height = STANDOFF_Y1 - STANDOFF_Y0
        tube = BRepAlgoAPI_Cut(BRepPrimAPI_MakeCylinder(axis, OUTER, height).Shape(),
                               BRepPrimAPI_MakeCylinder(axis, INNER, height).Shape())
        if not tube.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a standoff's hole")
        fuse = BRepAlgoAPI_Fuse(tray, tube.Shape())
        if not fuse.IsDone():
            raise Refused(f"BRepAlgoAPI_Fuse declined the standoff at (x, z) = ({x}, {z})")
        tray = fuse.Shape()
    return tray
