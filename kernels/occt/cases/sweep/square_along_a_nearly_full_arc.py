"""A square swept along 350° of an arc. A square of side 2 swept along 350° of an arc in the xy
plane about the z axis, of radius 10, from (10, 0, 0) anticlockwise seen from +z to
(10 cos 350°, 10 sin 350°, 0). The square starts in the xz plane from (9, 0, −1) to (11, 0, 1),
its sides parallel to the x and z axes. It stays square to the path, and the ends are flat: they
face each other across a gap of 10°."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp, gp_Circ, gp_Pnt

from bench import Refused

ARC_RADIUS = 10
HALF_SIDE = 1
SWEEP = 350  # degrees


def build():
    # The arc: the circle about the z axis in the xy plane (gp.XOY), from angle 0, at (10, 0, 0),
    # to 350°.
    arc = BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), ARC_RADIUS), 0,
                                  math.radians(SWEEP)).Edge()
    path = BRepBuilderAPI_MakeWire(arc).Wire()
    # The section where the arc starts, in the xz plane, square to the arc there.
    r, h = ARC_RADIUS, HALF_SIDE
    section = BRepBuilderAPI_MakePolygon()
    for x, z in ((r - h, -h), (r + h, -h), (r + h, h), (r - h, h)):
        section.Add(gp_Pnt(x, 0, z))
    section.Close()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)  # Frenet: the section keeps square to the path, its sides level and upright
    pipe.Add(section.Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the square")
    if not pipe.MakeSolid():  # close the ends with flat squares
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
