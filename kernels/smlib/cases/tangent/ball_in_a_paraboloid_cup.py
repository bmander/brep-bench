"""A ball in a paraboloid cup that fits its bottom. The solid paraboloid x² + y² ≤ 10z, z ≤ 5,
united with the ball of radius 5 about (0, 0, 5), whose curvature matches the cup's at its vertex:
the two touch there only, to more than first order."""
import usd_brep as sm

FOCAL = 2.5   # x² + y² = 4·FOCAL·z
TOP = 5       # the cup's top, a flat disk at z = TOP
RADIUS = 5    # of the ball, about (0, 0, RADIUS)


def cup():
    """The solid paraboloid: the parabola x² = 10z of the xz plane, from its vertex to the rim,
    and the top's radius, turned once about z."""
    rim = (4 * FOCAL * TOP) ** 0.5
    # a parabola's arc is a quadratic Bézier curve: its ends, and where their tangents meet
    arc = sm.create_canonical_curve([(0, 0, 0), (rim / 2, 0, 0), (rim, 0, TOP)], [0, 1], [3, 3],
                                    degree=2, form=sm.BSplineCurveForm.PARABOLIC_ARC)
    top = sm.create_line_segment((rim, 0, TOP), (0, 0, TOP))
    return sm.rotational_sweep([arc, top], (0, 0, 0), (0, 0, 1), 360)


def build():
    # SMLib's sphere has its poles on the z axis: its lower pole is the cup's vertex
    ball = sm.create_sphere((0, 0, RADIUS), RADIUS)
    return sm.boolean_union(cup(), ball)
