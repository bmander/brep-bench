"""A pin a tenth of a millimetre across, its rim rounded unequally by a quarter ellipse. A cylinder
of radius 0.05 on the z axis, from z = 0 to z = 0.1. Round the circle where its side meets its top
unequally: 0.01 in across the top and 0.03 down the side, with a section that is a quarter ellipse,
the points (ρ, z) = (0.04 + 0.01 cos θ, 0.07 + 0.03 sin θ), 0 ≤ θ ≤ 90°."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from rims import segment, turned

RADIUS, HEIGHT = 0.05, 0.1
ACROSS, DOWN = 0.01, 0.03


def build():
    # Workaround, as at a hundred times the size (fillet/elliptic-section-cylinder-rim): OCCT's
    # fillets have a circular section and it has no asymmetric (elliptic) round, so the pin is its
    # half section turned round the z axis: the quarter ellipse centred at (0.04, 0.07) as (x, z),
    # semi-axes 0.01 along x and 0.03 along z, closed by the axis, the bottom, the side and the
    # top.
    centre = gp_Pnt(RADIUS - ACROSS, 0, HEIGHT - DOWN)
    # The major axis along z and the frame's normal −y, so its other axis is −y × z = −x: the top
    # (0.04, 0.1) at t = 0 and the side (0.05, 0.07) at t = −π/2.
    frame = gp_Ax2(centre, gp_Dir(0, -1, 0), gp_Dir(0, 0, 1))
    ellipse = gp_Elips(frame, DOWN, ACROSS)
    quarter = BRepBuilderAPI_MakeEdge(ellipse, -math.pi / 2, 0).Edge()
    return turned([
        quarter,
        segment((RADIUS - ACROSS, 0, HEIGHT), (0, 0, HEIGHT)),
        segment((0, 0, HEIGHT), (0, 0, 0)),
        segment((0, 0, 0), (RADIUS, 0, 0)),
        segment((RADIUS, 0, 0), (RADIUS, 0, HEIGHT - DOWN)),
    ])
