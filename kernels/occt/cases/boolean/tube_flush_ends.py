"""A tube cut flush. A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Cut from it a
cylinder of radius 3 on the same axis that also runs exactly from z = 0 to z = 10. The ends of
the cutting cylinder lie in the ends of the first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from bench import Refused

OUTER_RADIUS = 5
INNER_RADIUS = 3
LENGTH = 10


def build():
    # Both cylinders stand on the origin along +z, so their ends are coplanar.
    body = BRepPrimAPI_MakeCylinder(OUTER_RADIUS, LENGTH).Shape()
    bore = BRepPrimAPI_MakeCylinder(INNER_RADIUS, LENGTH).Shape()
    cut = BRepAlgoAPI_Cut(body, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the flush bore")
    return cut.Shape()
