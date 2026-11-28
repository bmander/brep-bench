"""A bored cone frustum 3.1 times the size, shelled open at its top, the cavity a ring sealed below
it. The bored frustum of shell/bored-frustum-sealed-below-its-opening with every length times 3.1:
its base the disk of radius 18.6 at z = 0, its top the disk of radius 9.3 at z = 12.4, bored
through along the axis by a hole of radius 3.1, shelled inward to a wall 3.1 thick, open at its top.
The moved side and the moved bore meet below the top, so the cavity never reaches the opening: the
top stays as it was, and the cavity is a closed ring."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from offset import less
from shell import face_through, shell

SCALE = 3.1
BASE_RADIUS = 6 * SCALE
TOP_RADIUS = 3 * SCALE
HEIGHT = 4 * SCALE
BORE_RADIUS = 1 * SCALE
THICKNESS = 1 * SCALE


def build():
    frustum = BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    # The drill runs past both faces.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -SCALE), gp_Dir(0, 0, 1)), BORE_RADIUS,
                                     HEIGHT + 2 * SCALE).Shape()
    bored = less(frustum, drill, "the bore")
    # The frustum's edges are convex, where moved faces meet rather than part; joined by
    # intersection, the moved side and bore run on until they meet.
    top = face_through(bored, (0, (BORE_RADIUS + TOP_RADIUS) / 2, HEIGHT))
    return shell(bored, [top], THICKNESS, join=GeomAbs_Intersection)
