"""A lens a millionth thick. Two balls of radius 5, one centred on the origin and one at
(9.999999, 0, 0), so that they overlap to a depth of only 10⁻⁶. Make the lens common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS = 5
DISTANCE = 9.999999  # between the centres


def build():
    first = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    second = BRepPrimAPI_MakeSphere(gp_Pnt(DISTANCE, 0, 0), RADIUS).Shape()
    common = BRepAlgoAPI_Common(first, second)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two balls")
    return common.Shape()
