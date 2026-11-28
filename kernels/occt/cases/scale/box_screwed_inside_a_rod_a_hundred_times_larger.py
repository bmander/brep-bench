"""A sealed helical tunnel a hundred times larger: a box screwed inside a rod. A rod, the cylinder
of radius 500 about the z axis from z = 0 to z = 2000, with a sealed helical tunnel inside it:
everything the box 250 ≤ x ≤ 350, −50 ≤ y ≤ 50, 200 ≤ z ≤ 300 passes through as it turns right-
handed about the z axis, rising 400 for every whole turn, from a turn of 0° to one of 720°. Cut from
the rod everything the box passes through."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Pnt

from sweep import cut, flat_contacts_screwed, screwed

ROD_RADIUS, ROD_LENGTH = 500, 2000
BOX = ((250, -50, 200), (350, 50, 300))
LEAD = 400  # rise per whole turn
START, STOP = 0, 720  # degrees
# How far each plane splitting a face along its contact reaches: past the box, which stands 350
# from the axis.
REACH = 1000


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(*BOX[0]), gp_Pnt(*BOX[1])).Shape()
    # Each flat face is cut along its contact: the faces x = 250 and x = 350 along their middles,
    # y = 0, through the axis; the faces y = ±50 square to it nowhere on them; the level faces
    # rise or sink all over.
    pieces = screwed(box, LEAD, START, STOP, flat_contacts_screwed(box, LEAD, REACH))
    return cut(BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape(), pieces)
