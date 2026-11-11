"""A hollow square duct elbow. A hollow square duct bent through a quarter turn, its centreline an
arc of radius 30 in the xy plane about the z axis, from (30, 0, 0) to (0, 30, 0). Its section is
a square 18 × 18 centred on the arc with a square hole 14 × 14 at its centre, the wall 2 thick:
the duct is that hollow square, standing in the xz plane at x = 30, turned a quarter turn about
the z axis, from +x toward +y. Its two ends are flat."""
import usd_brep as sm

BEND_RADIUS = 30
HALF_OUTSIDE, HALF_HOLE = 9, 7
TURN = 90   # degrees


def square(half):
    """The square of half-width `half` in the xz plane, centred at (30, 0, 0), as four lines."""
    corners = [(BEND_RADIUS + x, 0, z)
               for x, z in [(-half, -half), (half, -half), (half, half), (-half, half)]]
    return [sm.create_line_segment(a, b) for a, b in zip(corners, corners[1:] + corners[:1])]


def build():
    # The section, its outside and its hole, turned about the z axis: by the right-hand rule,
    # from +x toward +y. The two ends are capped.
    return sm.rotational_sweep(square(HALF_OUTSIDE) + square(HALF_HOLE),
                               (0, 0, 0), (0, 0, 1), TURN)
