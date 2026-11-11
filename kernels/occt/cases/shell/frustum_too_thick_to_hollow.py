"""A cone frustum shelled open at its top, so thick that nothing is hollowed. A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk
of radius 3 at z = 4. Shell it inward to a wall 3.5 thick, open at its top face, z = 4. The side,
moved 3.5 inward, closes at (0, 0, 13/6), below the base moved up to z = 3.5: the cavity is empty,
nothing is hollowed, and the result is the frustum itself."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection

from shell import face_through, shell

BASE_RADIUS = 6
TOP_RADIUS = 3
HEIGHT = 4
THICKNESS = 3.5


def build():
    frustum = BRepPrimAPI_MakeCone(BASE_RADIUS, TOP_RADIUS, HEIGHT).Shape()
    # A frustum's edges are convex, where moved faces meet rather than part; joined by
    # intersection, as the cone's apex is, the moved side runs on as its cone.
    return shell(frustum, [face_through(frustum, (1, 1, HEIGHT))], THICKNESS, join=GeomAbs_Intersection)
