"""A hollow square reducer. A hollow square tube that tapers along the y axis, from y = 0 to
y = 40.

Its sections are squares centred on the y axis, with sides parallel to the x and z axes, each with
a square hole at its centre leaving a wall 2 thick. At y = 0 the outside is 24 × 24 (x and z from
−12 to 12) and the hole 20 × 20; at y = 40 the outside is 12 × 12 and the hole 8 × 8. In
between, the sizes run linearly with y, so each corner, outside and in, runs in a straight line
from one end to the other and the eight long faces are flat trapezoids. The two ends are flat."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused

LENGTH = 40
OUTER_START, OUTER_END = 24, 12
INNER_START, INNER_END = 20, 8


def square(side, y):
    """A square of the given side centred on the y axis at y, its sides along x and z."""
    h = side / 2
    poly = BRepBuilderAPI_MakePolygon()
    for x, z in [(-h, -h), (h, -h), (h, h), (-h, h)]:
        poly.Add(gp_Pnt(x, y, z))
    poly.Close()
    return poly.Wire()


def frustum(start, end):
    """The solid square frustum from a square of side `start` at y = 0 to `end` at y = LENGTH:
    a ruled loft, so corner runs to corner in a straight line."""
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(square(start, 0))
    loft.AddWire(square(end, LENGTH))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the squares")
    return loft.Shape()


def build():
    # The hole runs exactly from end to end, so its ends lie in the tube's.
    cut = BRepAlgoAPI_Cut(frustum(OUTER_START, OUTER_END), frustum(INNER_START, INNER_END))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the hole")
    return cut.Shape()
