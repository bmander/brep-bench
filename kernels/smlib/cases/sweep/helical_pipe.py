"""A helical pipe. A pipe of circular section, radius 1, following a helix about the z axis. The
helix has radius 10 and rises 5 per turn. It starts at (10, 0, 0), turns right-handed (from +x
toward +y) and makes three whole turns. The section stays square to the path, and the pipe's
ends are flat."""
import usd_brep as sm

HELIX_RADIUS = 10
PITCH = 5   # rise per turn
TURNS = 3
PIPE_RADIUS = 1
HELIX_TOLERANCE = 1e-6   # the helix is a B-spline fitted to within this, the tightest it takes


def build():
    # A helix stands on the z axis through its origin and starts on the +x side.
    helix = sm.create_helix((0, 0, 0), PITCH * TURNS, HELIX_RADIUS, HELIX_RADIUS, TURNS,
                            right_handed=True, tolerance=HELIX_TOLERANCE)
    # A pipe: a circle square to the path all along it, its ends capped flat.
    return sm.pipe_sweep(helix, radius=PIPE_RADIUS)
