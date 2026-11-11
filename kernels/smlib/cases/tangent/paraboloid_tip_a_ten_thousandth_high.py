"""A paraboloid's tip a ten-thousandth high. The solid paraboloid x² + y² ≤ 8z, z ≤ 8, in common
with the half-space below the plane z = 10⁻⁴: a disk 0.057 across over a bowl 10⁻⁴ deep."""
import usd_brep as sm

FOCAL = 2       # x² + y² = 4·FOCAL·z
TOP = 8         # the paraboloid's top, a flat disk at z = TOP
CUT_Z = 1e-4
PAST = 1        # how far the block reaches past the paraboloid, on every side but its top


def paraboloid(rim):
    """The solid paraboloid: the parabola x² = 8z of the xz plane, from its vertex to the rim, and
    the top's radius, turned once about z."""
    # a parabola's arc is a quadratic Bézier curve: its ends, and where their tangents meet
    arc = sm.create_canonical_curve([(0, 0, 0), (rim / 2, 0, 0), (rim, 0, TOP)], [0, 1], [3, 3],
                                    degree=2, form=sm.BSplineCurveForm.PARABOLIC_ARC)
    top = sm.create_line_segment((rim, 0, TOP), (0, 0, TOP))
    return sm.rotational_sweep([arc, top], (0, 0, 0), (0, 0, 1), 360)


def build():
    rim = (4 * FOCAL * TOP) ** 0.5
    reach = rim + PAST
    # the half-space, as a block whose top is the plane
    below = sm.create_box((-reach, -reach, -PAST), 2 * reach, 2 * reach, PAST + CUT_Z)
    return sm.boolean_intersection(paraboloid(rim), below)
