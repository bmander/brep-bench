"""A torus: a solid torus centred on the origin, about the z axis. The circle through the middle
of its tube has radius 10, and the tube has radius 2."""
import usd_brep as sm

CENTRE = (0, 0, 0)
MAJOR_RADIUS = 10  # the circle through the middle of the tube
MINOR_RADIUS = 2   # the tube


def build():
    # SMLib's torus has its middle circle in the xy plane: it is about z.
    return sm.create_torus(CENTRE, MAJOR_RADIUS, MINOR_RADIUS)
