"""A paraboloid's tip on a face. A block from (−10, −10, 0) to (10, 10, 10), united with the solid
paraboloid x² + y² ≤ 8z, z ≤ 12.5, whose vertex is at the origin, the middle of the block's bottom
face."""
import usd_brep as sm

HALF, THICKNESS = 10, 10
FOCAL = 2       # x² + y² = 4·FOCAL·z
TOP = 12.5      # the paraboloid's top, a flat disk at z = TOP


def paraboloid():
    """The solid paraboloid: the parabola x² = 8z of the xz plane, from its vertex to the rim, and
    the top's radius, turned once about z."""
    rim = (4 * FOCAL * TOP) ** 0.5
    # a parabola's arc is a quadratic Bézier curve: its ends, and where their tangents meet
    arc = sm.create_canonical_curve([(0, 0, 0), (rim / 2, 0, 0), (rim, 0, TOP)], [0, 1], [3, 3],
                                    degree=2, form=sm.BSplineCurveForm.PARABOLIC_ARC)
    top = sm.create_line_segment((rim, 0, TOP), (0, 0, TOP))
    return sm.rotational_sweep([arc, top], (0, 0, 0), (0, 0, 1), 360)


def build():
    block = sm.create_box((-HALF, -HALF, 0), 2 * HALF, 2 * HALF, THICKNESS)
    return sm.boolean_union(block, paraboloid())
