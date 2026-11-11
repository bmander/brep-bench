"""A bored cone frustum shelled open at its top, the cavity a ring sealed below it. A solid cone
frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of radius 3 at
z = 4, bored through along the axis by a hole of radius 1. Shell it inward to a wall 1 thick, open
at its top, the annulus between radii 1 and 3. The moved side and the moved bore meet in the circle
of radius 2 at z = 11/3, below the top, so the cavity never reaches the opening: the top stays as it
was, and the result is the bored frustum less the closed ring whose section is the triangle (2, 1),
(4, 1), (2, 11/3) in (ρ, z)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from shell import face_through, shell

BASE_RADIUS = 6
TOP_RADIUS = 3
HEIGHT = 4
BORE_RADIUS = 1
THICKNESS = 1


def build():
    frustum = BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    # The drill runs past both faces.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), BORE_RADIUS, HEIGHT + 2).Shape()
    bored = BRepAlgoAPI_Cut(frustum, drill).Shape()
    # The frustum's edges are convex, where moved faces meet rather than part; joined by
    # intersection, the moved side and bore run on until they meet.
    return shell(bored, [face_through(bored, (0, 2, HEIGHT))], THICKNESS, join=GeomAbs_Intersection)
