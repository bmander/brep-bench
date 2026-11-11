"""A cone frustum shelled open at its top, the cavity closing to a point below it. A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of
radius 3 at z = 4. Shell it inward to a wall 2.7 thick, open at its top face, z = 4. The side, moved
2.7 inward, closes to its apex at (0, 0, 3.5), below the top, so the cavity never reaches the opening:
the top stays as it was, and the result is the frustum less the closed cone from the disk of radius
0.6 at z = 2.7 to (0, 0, 3.5)."""
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
    return shell(frustum, [face_through(frustum, (1, 1, HEIGHT))], THICKNESS, join=GeomAbs_Intersection)
