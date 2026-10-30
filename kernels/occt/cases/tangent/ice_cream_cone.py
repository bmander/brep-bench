"""An ice-cream cone. A ball of radius 15 centred on the origin, united with a solid cone on the
z axis whose apex is at (0, 0, −25) and whose flat base is the disc of radius 12 in the plane
z = −9. The cone's side is tangent to the sphere along the rim of its base."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

BALL_RADIUS = 15
APEX_Z = -25
BASE_Z = -9
BASE_RADIUS = 12


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), BALL_RADIUS).Shape()
    # from the apex (radius 0) up to the base
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, APEX_Z), gp.DZ_s()),
                                0, BASE_RADIUS, BASE_Z - APEX_Z).Shape()
    fuse = BRepAlgoAPI_Fuse(ball, cone)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball and the cone")
    return fuse.Shape()
