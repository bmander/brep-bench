"""A bored mounting flange with six bolt holes. A round mounting flange on the z axis, its base on
the plane z = 0:

- a plate, a disc of radius 32 from z = 0 to z = 6;
- a hub, a cylinder of radius 15 standing on the plate from z = 6 to z = 16;
- a bore of radius 8 on the z axis, right through the plate and the hub from z = 0 to z = 16.

Six bolt holes of radius 3 go straight through the plate along z. Their axes stand on a circle of
radius 24 about the z axis, one every 60°: at (x, y) = (24, 0), (12, 12√3), (−12, 12√3), (−24, 0),
(−12, −12√3) and (12, −12√3). The holes stay clear of the hub (their nearest point is 21 from the
axis, the hub's side 15) and of the plate's rim (furthest point 27, rim 32). If you cut the holes
with cylinders, make them run past both faces of the plate."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

PLATE_RADIUS, PLATE_TOP = 32, 6
HUB_RADIUS, HUB_TOP = 15, 16
BORE_RADIUS = 8
BOLT_CIRCLE, BOLT_RADIUS, BOLTS = 24, 3, 6


def cylinder(x, y, z0, radius, height):
    """A cylinder along z with its axis through (x, y), from z0 to z0 + height."""
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z0), gp.DZ_s()), radius, height).Shape()


def build():
    plate = cylinder(0, 0, 0, PLATE_RADIUS, PLATE_TOP)
    hub = cylinder(0, 0, PLATE_TOP, HUB_RADIUS, HUB_TOP - PLATE_TOP)
    fuse = BRepAlgoAPI_Fuse(plate, hub)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the hub on the plate")
    flange = fuse.Shape()
    # The bore and the holes run 1 past the faces they go through.
    cutters = [cylinder(0, 0, -1, BORE_RADIUS, HUB_TOP + 2)]
    for i in range(BOLTS):
        angle = math.radians(60 * i)
        cutters.append(cylinder(BOLT_CIRCLE * math.cos(angle), BOLT_CIRCLE * math.sin(angle), -1,
                                BOLT_RADIUS, PLATE_TOP + 2))
    for cutter in cutters:
        cut = BRepAlgoAPI_Cut(flange, cutter)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined the bore or a bolt hole")
        flange = cut.Shape()
    return flange
