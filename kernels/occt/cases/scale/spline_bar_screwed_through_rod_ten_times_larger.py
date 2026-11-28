"""A rod threaded by a bar whose section is a spline, ten times larger. A rod, the cylinder of
radius 50 about the z axis from z = 0 to z = 200, threaded by a bar whose section, in the plane y =
0, is the closed uniform periodic cubic B-spline through the control points (35, 0), (45, −15), (80,
−15), (120, −10), (120, 10), (80, 15) and (45, 15) in (x, z), carried from y = −80 to y = 80. The
bar is screwed about the z axis, rising 60 a turn, from a turn of −90° to one of 1350°; cut from the
rod everything it passes through."""
from bench import Refused


def build():
    # As at a tenth of the size (swept/spline-bar-screwed-through-rod): OCCT makes the bar, a
    # prism of the spline, exactly, but its side touches what the screw sweeps along a curve with
    # no closed form, found only numerically at each ruling. OCCT sweeps no solid, and a face it
    # carries along the screw sweeps to a solid only once split along its contact: finding that
    # curve would be computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it carries faces along a screw, and the spline "
                  "bar's side touches what it sweeps along a curve with no closed form, which "
                  "occt has no operation to find")
