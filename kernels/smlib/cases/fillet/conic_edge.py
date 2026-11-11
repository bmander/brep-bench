"""A block's edge blended by a parabolic conic. A block, the box from the origin to (20, 10, 10).
Blend the edge where its top, z = 10, meets its end, x = 20, with a conic blend of rho 0.5, a
parabola, tangent to the top and to the end each 3 from the edge: in every plane y = c the
quadratic Bézier curve through (17, c, 10), (20, c, 10) and (20, c, 7)."""
import usd_brep as sm

BLOCK = (20, 10, 10)
SETBACK = 3
MARGIN = 1


def build():
    # Workaround: SMLib's fillets have a circular or a flat section, or its own blend curve
    # shaped by a continuity and a fullness, and none takes a conic's rho. The same solid is the
    # block less the corner outside the parabola: the parabola, closed around the corner outside
    # the block, swept along y past both ends.
    length, width, height = BLOCK
    y = -MARGIN
    on_top, corner = (length - SETBACK, y, height), (length, y, height)
    on_end = (length, y, height - SETBACK)
    parabola = sm.create_canonical_curve([on_top, corner, on_end], [0, 1], [3, 3], degree=2,
                                         form=sm.BSplineCurveForm.PARABOLIC_ARC)
    around = [on_end, (length + MARGIN, y, height - SETBACK), (length + MARGIN, y, height + MARGIN),
              (length - SETBACK, y, height + MARGIN), on_top]
    outline = [parabola] + [sm.create_line_segment(a, b) for a, b in zip(around, around[1:])]
    outside = sm.linear_sweep(outline, (0, 1, 0), width + 2 * MARGIN)
    return sm.boolean_difference(sm.create_box((0, 0, 0), length, width, height), outside)
