"""A ball in a band that touches its equator. A ball of radius 5 centred on the origin, united
with a cylinder of the same radius on the z axis, from z = −3 to z = 3. Inside the band the ball
touches the cylinder's wall from inside along the equator, where the two are tangent."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS = 5
HALF_HEIGHT = 3


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    band = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -HALF_HEIGHT), gp.DZ_s()),
                                    RADIUS, 2 * HALF_HEIGHT).Shape()
    fuse = BRepAlgoAPI_Fuse(ball, band)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the ball and the band")
    return fuse.Shape()
