"""A cone frustum shelled open at both ends, the cavity closing to a point below its top. A solid cone
frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of radius 3 at
z = 4. Shell it inward to a wall 2.7 thick, open at both its base and its top. The side, moved 2.7
inward, meets the base's plane in the circle of radius 2.625 and closes to its apex at (0, 0, 3.5),
below the top, so the top is never reached: the result is the frustum less the cone from the disk
of radius 2.625 at z = 0 to (0, 0, 3.5), open at the base only."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection

from shell import face_through, shell

BASE_RADIUS = 6
TOP_RADIUS = 3
HEIGHT = 4
THICKNESS = 2.7


def build():
    frustum = BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    # A frustum's edges are convex, where moved faces meet rather than part; joined by
    # intersection, as the cone's apex is, the moved side runs on as its cone.
    openings = [face_through(frustum, (1, 1, 0)), face_through(frustum, (1, 1, HEIGHT))]
    return shell(frustum, openings, THICKNESS, join=GeomAbs_Intersection)
