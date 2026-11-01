"""A cone shelled open at its base, its apex moved in. A solid cone on the z axis, its base the disc of
radius 6 at z = 0 and its apex at (0, 0, 8), so its side is 10 long from the base to the apex and
its half-angle α has sin α = 3/5. Shell it inward to a wall 1 thick, open at its base, z = 0. The
outside of the wall is its side, unmoved, and the inside is the side moved 1 inward along its
normal, onto the surface offset 1 from it. The moved faces are extended or trimmed where they meet
one another, and run on until they meet the surface of an opening, which is left as a rim between
the outside and the inside."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.GeomAbs import GeomAbs_Intersection

from shell import face_through, shell

BASE_RADIUS = 6
HEIGHT = 8
THICKNESS = 1


def build():
    cone = BRepPrimAPI_MakeCone(BASE_RADIUS, 0, HEIGHT).Shape()
    # A cone's edges and apex are convex, where moved faces meet rather than part, so the join
    # makes no round and either kind gives the same wall; joined by intersection, the apex moves
    # with the side.
    return shell(cone, [face_through(cone, (0, 0, 0))], THICKNESS, join=GeomAbs_Intersection)
