"""A torus ten thousand times larger cut by a plane tangent to it at two points. A torus about the z
axis, centred on the origin, of radii 50000 (its middle circle) and 30000 (its tube), cut by the
plane 3x − 4z = 0, keeping every point with z ≤ 3x/4. The plane is tangent to the torus at two
points and cuts it in Villarceau's circles."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

MAJOR_RADIUS = 50000
MINOR_RADIUS = 30000


def build():
    # the seam meridian in the plane x = 0, on the side y > 0, away from the tangent points
    frame = gp_Ax2(gp.Origin_s(), gp.DZ_s(), gp.DY_s())
    torus = BRepPrimAPI_MakeTorus(frame, MAJOR_RADIUS, MINOR_RADIUS).Shape()
    # what is cut away is the half-space above the plane, on the side of its normal (−3, 0, 4)
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp.Origin_s(), gp_Dir(-3, 0, 4))).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(-30000, 0, 40000)).Solid()
    cut = BRepAlgoAPI_Cut(torus, above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    return cut.Shape()
