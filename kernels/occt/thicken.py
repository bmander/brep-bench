"""What the OCCT implementations of the thicken cases share: thickening a sheet (a face, or a shell
of faces that is not closed) into a solid, with BRepOffset_MakeOffset in its thickening mode.

The sheet's faces are each moved `distance` along their normals, and the solid is what lies between
the sheet and the moved faces: a positive distance thickens it to the side its faces' normals
point to, a negative one to the other side. Where the faces of a sheet meet at a crease and the
moved faces part, they are joined by a round about the crease (GeomAbs_Arc) or extended until they
meet (GeomAbs_Intersection); where they cross, they are trimmed where they meet, either way.
"""
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.GeomAbs import GeomAbs_Arc

from bench import Refused

TOLERANCE = 1e-7


def thicken(sheet, distance, join=GeomAbs_Arc):
    """The solid between the sheet and its faces moved by `distance`; Refused if OCCT declines."""
    maker = BRepOffset_MakeOffset()
    # Skin mode moves every face. Intersection = True intersects every moved face with every other,
    # not only with those beside it; Thickening = True makes the solid between the sheet and them.
    maker.Initialize(sheet, distance, TOLERANCE, BRepOffset_Skin, True, False, join, True)
    maker.MakeOffsetShape()
    if not maker.IsDone():
        raise Refused(f"BRepOffset_MakeOffset declined to thicken the sheet by {distance}")
    return maker.Shape()
