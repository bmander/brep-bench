"""A round pipe turned around a sharp corner with a round joint. The path: two straight runs, from
(−30, 0, 0) along +x to the origin, then turning a right angle to the left and on along +y to
(0, 30, 0). The section: a disc of radius 3, square to the path and centred on it. Each run is the
disc swept straight along it, ending flat at the corner; the wedge they leave open outside the
corner is filled by the disc turned about the z axis from one run to the other, a quarter of the
ball of radius 3 about the origin. So the pipe is every point within 3 of the path, cut off flat
at its ends."""
import usd_brep as sm

RUN = 30
RADIUS = 3


def build():
    # Workaround: SMLib's sweep along a cornered path mitres its corners and puts no round joint
    # in (sweep_along_planar_path), and its pipe_sweep declines a path with a corner. The same
    # solid is the two runs, each a cylinder ending flat at the corner, and the ball about the
    # corner: the ball's quarter outside the corner fills the wedge, and the rest of it lies
    # inside the runs.
    # A cylinder stands on the z axis: each run is made there and laid along its own axis.
    def run(axis, turn):
        return sm.rotate(sm.create_cylinder((0, 0, 0), RADIUS, RUN), (0, 0, 0), axis, turn)
    first = run((0, 1, 0), -90)    # from the corner back along −x
    second = run((1, 0, 0), -90)   # from the corner on along +y
    pipe = sm.boolean_union(first, second)
    return sm.boolean_union(pipe, sm.create_sphere((0, 0, 0), RADIUS))
