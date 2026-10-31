"""What the OCCT implementations of the offset cases share: growing or shrinking a solid by a
distance, with BRepOffsetAPI_MakeOffsetShape.

A positive distance grows the solid and a negative one shrinks it. Joined by arcs (GeomAbs_Arc),
the faces moved apart are joined by rounds and the faces moved through each other are trimmed, so
the result is every point within the distance of the solid, or at least that far inside it. Joined
by intersection (GeomAbs_Intersection), the moved faces are extended until they meet, and every
edge stays sharp.
"""
from OCP.BRepOffset import BRepOffset_Skin
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape
from OCP.GeomAbs import GeomAbs_Arc

from bench import Refused

TOLERANCE = 1e-7


def offset(solid, distance, join=GeomAbs_Arc):
    """The solid grown by `distance` (shrunk where it is negative); Refused if OCCT declines."""
    maker = BRepOffsetAPI_MakeOffsetShape()
    # Skin mode offsets every face. Intersection = True intersects every moved face with every
    # other, not only with those beside it, so moved faces that cross faces across a groove or a
    # flange are trimmed too.
    maker.PerformByJoin(solid, distance, TOLERANCE, BRepOffset_Skin, True, False, join)
    if not maker.IsDone():
        raise Refused(f"BRepOffsetAPI_MakeOffsetShape declined to offset the solid by {distance}")
    return maker.Shape()
