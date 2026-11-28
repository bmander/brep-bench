"""A circle swept along 350° of an arc. A circle of radius 2 swept along 350° of an arc in the xy
plane about the z axis, of radius 10, from (10, 0, 0) anticlockwise seen from +z to
(10 cos 350°, 10 sin 350°, 0). The section stays square to the path, and the ends are flat: they
face each other across a gap of 10°."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Pnt

from bench import Refused

ARC_RADIUS = 10
SECTION_RADIUS = 2
SWEEP = 350  # degrees


def build():
    # The arc: the circle about the z axis in the xy plane (gp.XOY), from angle 0, at (10, 0, 0),
    # to 350°.
    arc = BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), ARC_RADIUS), 0,
                                  math.radians(SWEEP)).Edge()
    path = BRepBuilderAPI_MakeWire(arc).Wire()
    # The section where the arc starts, square to it: the arc leaves (10, 0, 0) along +y.
    section = BRepBuilderAPI_MakeEdge(
        gp_Circ(gp_Ax2(gp_Pnt(ARC_RADIUS, 0, 0), gp.DY_s()), SECTION_RADIUS)).Edge()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)  # Frenet: the section keeps square to the path
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle")
    if not pipe.MakeSolid():  # close the ends with flat discs
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
