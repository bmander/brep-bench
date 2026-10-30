"""A disc with six holes indexed around it. A round disc of radius 20 on the z axis, from z = −5
to z = 0. Six round holes of radius 2 go straight through it along z, their axes on the circle of
radius 12 about the z axis: the first at (12, 0), and each of the others the first turned about
the z axis by 60°, 120°, 180°, 240° and 300°. Cut them one after another, each hole its own cut.
If you cut a hole with a cylinder, make it run past both faces of the disc."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt, gp_Trsf

from bench import Refused

DISC_RADIUS = 20
BOTTOM, TOP = -5, 0
HOLE_RADIUS = 2
PITCH_RADIUS = 12
COUNT = 6


def build():
    disc = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, BOTTOM), gp.DZ_s()),
                                    DISC_RADIUS, TOP - BOTTOM).Shape()
    # The first drill, at (12, 0), runs 1 past both faces: from z = −6 to z = 1.
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(PITCH_RADIUS, 0, BOTTOM - 1), gp.DZ_s()),
                                     HOLE_RADIUS, TOP - BOTTOM + 2).Shape()
    for i in range(COUNT):
        turn = gp_Trsf()
        turn.SetRotation(gp.OZ_s(), math.radians(i * 360 / COUNT))
        drill = BRepBuilderAPI_Transform(first, turn, True).Shape()
        cut = BRepAlgoAPI_Cut(disc, drill)
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined hole {i}")
        disc = cut.Shape()
    return disc
