"""An ellipsoid halved. The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 (semi-axes 6 along x, 4 along y and 3
along z, centred on the origin), with everything below the plane z = 0 cut away. What is left is its
upper half: a dome standing on the flat ellipse x²/36 + y²/16 ≤ 1 in the plane z = 0, rising to 3 at
its top.

The cut runs through the ellipsoid's widest section, where its surface stands upright, so the dome
meets its floor at a right angle all round. If you cut with a block, make it reach past the
ellipsoid everywhere but at z = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_GTransform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_GTrsf, gp_Mat, gp_Pnt, gp_XYZ

from bench import Refused

A, B, C = 6, 4, 3  # the semi-axes along x, y and z


def build():
    ball = BRepPrimAPI_MakeSphere(1).Shape()
    stretch = gp_GTrsf(gp_Mat(A, 0, 0, 0, B, 0, 0, 0, C), gp_XYZ(0, 0, 0))
    ellipsoid = BRepBuilderAPI_GTransform(ball, stretch, True).Shape()
    # the half above z = 0: in common with a block on that plane reaching past it elsewhere
    above = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 10)).Shape()
    half = BRepAlgoAPI_Common(ellipsoid, above)
    if not half.IsDone():
        raise Refused("BRepAlgoAPI_Common declined to cut the ellipsoid at z = 0")
    return half.Shape()
