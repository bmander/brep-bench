"""A round pipe mitred along three edges of a cube. The path: three straight runs along three
edges of a cube of side 30, from (0, 0, 0) to (30, 0, 0), to (30, 30, 0), to (30, 30, 30); the
second corner turns out of the plane of the first two runs. The section: a disc of radius 2,
square to the path and centred on it. At each corner the two runs are mitred, meeting in the
plane through the corner that bisects it. The pipe's ends are flat and square to the path."""
import usd_brep as sm

SIDE = 30
RADIUS = 2
CORNERS = [(0, 0, 0), (SIDE, 0, 0), (SIDE, SIDE, 0), (SIDE, SIDE, SIDE)]


def disc_rim(centre, turn_axis, turn):
    """The circle of the pipe's radius about `centre`, turned from the xy plane, where a circle
    is made, by `turn` degrees about `turn_axis`."""
    circle = sm.create_circle((0, 0, 0), RADIUS)
    sm.rotate(circle, (0, 0, 0), turn_axis, turn)
    return sm.translate(circle, centre)


def elbow(rim, corners):
    """The disc swept along the two runs through three corners, mitred where they meet."""
    path = [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:])]
    return sm.sweep_along_planar_path([rim], path)


def build():
    # Workaround: SMLib mitres a sweep only along a path in one plane (sweep_along_planar_path),
    # and its sweeps along a curve in space (pipe_sweep, curve_sweep) decline a path with a
    # corner. The pipe is made as two mitred elbows joined, one in the plane z = 0 along the
    # first two runs and one in the plane x = 30 along the last two. They share the middle run.
    # Each elbow's middle run goes on, flat-ended, past the other's mitre, and that stub lies
    # inside the other elbow: beyond the mitre plane it is nearer the next run's axis than its
    # own, both pipes being of one radius and square to each other.
    first = elbow(disc_rim(CORNERS[0], (0, 1, 0), 90), CORNERS[:3])    # the disc square to x
    second = elbow(disc_rim(CORNERS[1], (1, 0, 0), -90), CORNERS[1:])  # the disc square to y
    return sm.boolean_union(first, second)
