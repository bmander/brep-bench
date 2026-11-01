"""An open box's sheet thickened inward. The floor and four walls of the cube from the origin to
(10, 10, 10), with no top, thickened by 1 into the cube."""
from OCP.BRep import BRep_Builder
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopoDS import TopoDS_Shell

from thicken import thicken

SIDE = 10
THICKNESS = 1


def sheet():
    """The cube's floor and four walls, without its top, as one open shell."""
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE)
    tray, builder = TopoDS_Shell(), BRep_Builder()
    builder.MakeShell(tray)
    for face in (cube.BottomFace(), cube.FrontFace(), cube.BackFace(), cube.LeftFace(),
                 cube.RightFace()):
        builder.Add(tray, face)
    return tray


def build():
    # the cube's faces' normals point out of it
    return thicken(sheet(), -THICKNESS)
