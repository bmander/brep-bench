"""A ball dimpled by a small ball, the dimple's rim a hair from the small ball's pole. A ball of
radius 10 centred on the origin, less a ball of radius 0.8 whose centre is 10.4 from the origin, at
height z = 5.501, on the half-plane that leaves the z axis at 120° from +x toward +y; each ball's
poles on the vertical through its centre, so the circle where they meet passes about 0.00009 from
the small ball's lower pole."""
from math import cos, radians, sin, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 10
DIMPLE_RADIUS = 0.8
DIMPLE_DISTANCE = 10.4  # of the small ball's centre from the origin
DIMPLE_HEIGHT = 5.501
DIMPLE_BEARING = radians(120)  # from +x toward +y


def build():
    rho = sqrt(DIMPLE_DISTANCE ** 2 - DIMPLE_HEIGHT ** 2)
    centre = gp_Pnt(rho * cos(DIMPLE_BEARING), rho * sin(DIMPLE_BEARING), DIMPLE_HEIGHT)
    # OCCT's spheres made about a point have their poles on the vertical through it
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    dimple = BRepPrimAPI_MakeSphere(centre, DIMPLE_RADIUS).Shape()
    cut = BRepAlgoAPI_Cut(ball, dimple)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the small ball")
    return cut.Shape()
