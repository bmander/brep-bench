"""A horn torus. A solid horn torus centred on the origin, about the z axis: the circle through the
middle of its tube has radius 5, and the tube has radius 5 too. It is every point within 5 of that
circle, the disc (x − 5)² + z² ≤ 25 of the xz plane turned a whole turn about the z axis.

The tube's radius equals the middle circle's, so the tube closes its hole to a single point: the
origin, where the disc touches the axis. There the surface pinches, every meridian's circle passing
through it: what was the hole is two dimples, above and below, each a cusp narrowing to the axis
(its radius about z²/10 near the origin), whose tips meet there. It is one solid, with no hole
through it."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

MAJOR_RADIUS = 5  # the circle through the middle of the tube
MINOR_RADIUS = 5  # the tube


def build():
    return BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR_RADIUS, MINOR_RADIUS).Shape()
