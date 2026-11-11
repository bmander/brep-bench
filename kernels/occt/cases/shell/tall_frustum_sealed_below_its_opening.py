"""A tall cone frustum shelled open at its narrow top, the cavity a cone sealed well below it. A solid cone frustum on the z axis, its base the disk of radius 7 at z = 0 and its top the
disk of radius 1 at z = 8. Shell it inward to a wall 2 thick, open at its top face, z = 8. The side,
moved 2 inward, closes to its apex at (0, 0, 6), 2 below the top, so the cavity never reaches the
opening: the top stays as it was, and the result is the frustum less the closed cone from the disk
of radius 3 at z = 2 to (0, 0, 6)."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection

from shell import face_through, shell

BASE_RADIUS = 7
TOP_RADIUS = 1
HEIGHT = 8
THICKNESS = 2


def build():
    frustum = BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    # A frustum's edges are convex, where moved faces meet rather than part; joined by
    # intersection, as the cone's apex is, the moved side runs on as its cone.
    return shell(frustum, [face_through(frustum, (0.5, 0.5, HEIGHT))], THICKNESS, join=GeomAbs_Intersection)
