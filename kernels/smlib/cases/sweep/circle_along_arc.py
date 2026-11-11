"""A circle swept along an arc. A circle of radius 2 swept along a quarter turn of an arc. The arc
lies in the xy plane about the z axis, with radius 10, and runs from (10, 0, 0) to (0, 10, 0).
The section stays square to the path, and the ends are flat."""
import usd_brep as sm

ARC_RADIUS = 10
SECTION_RADIUS = 2


def build():
    # An arc lies in the xy plane, its angles measured from +x toward +y.
    arc = sm.create_arc((0, 0, 0), ARC_RADIUS, 0, 90)
    # A pipe: a circle square to the path all along it, its ends capped flat.
    return sm.pipe_sweep(arc, radius=SECTION_RADIUS)
