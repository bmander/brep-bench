"""A cone spun about a line that crosses it aslant, ten times larger. The cone's axis is the line
through the origin along w = (cos 30°, 0, sin 30°); its point is at −20w and it opens along +w at
50° to a flat base at +50w, of radius 70 tan 25°. It turns rigidly about the z axis, right-handed,
by 150°. Make the solid of everything the cone passes through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from sweep import plane, turned, united

TILT = math.radians(30)  # of the cone's axis w, up from the x axis toward z
POINT, BASE = -20, 50  # along w, from the origin
HALF_ANGLE = math.radians(25)
TURN = 150  # degrees about the z axis


def build():
    w = (math.cos(TILT), 0, math.sin(TILT))
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(*(POINT * x for x in w)), gp_Dir(*w)), 0,
                                (BASE - POINT) * math.tan(HALF_ANGLE), BASE - POINT).Shape()
    # As at a tenth of the size (swept/cone-spun-about-a-line-through-it): the side meets the turn
    # square-on in the plane y = 0 through both axes, and on the circle whose normals pass through
    # the origin, at s = 20·(−sin² 25°) along w; the flat base along y = 0 too. The plane x = 0
    # through the turn's axis cuts the cone where that axis leaves it, so that no piece turned
    # crosses the axis.
    circle = POINT * math.sin(HALF_ANGLE) ** 2
    cuts = [plane((0, 0, 0), (0, 1, 0)), plane(tuple(circle * x for x in w), w),
            plane((0, 0, 0), (1, 0, 0))]
    return united(turned(cone, gp.OZ_s(), TURN, cuts))
