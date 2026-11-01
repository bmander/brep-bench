"""An ellipsoid. A solid ellipsoid centred on the origin, its three semi-axes along the coordinate
axes: 6 along x, 4 along y and 3 along z. It is every point with x²/36 + y²/16 + z²/9 ≤ 1.

No two of its semi-axes are equal, so it is not a surface of revolution: no circle and no turn about
an axis makes it. A kernel with no ellipsoid may make it as a ball of radius 1 stretched by 6, 4 and
3 along x, y and z."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_GTransform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_GTrsf, gp_Mat, gp_XYZ

A, B, C = 6, 4, 3  # the semi-axes along x, y and z


def build():
    # OCCT has no ellipsoid: the unit ball stretched by a non-uniform scale, which GTransform
    # carries out exactly, as a rational B-spline
    ball = BRepPrimAPI_MakeSphere(1).Shape()
    stretch = gp_GTrsf(gp_Mat(A, 0, 0, 0, B, 0, 0, 0, C), gp_XYZ(0, 0, 0))
    return BRepBuilderAPI_GTransform(ball, stretch, True).Shape()
