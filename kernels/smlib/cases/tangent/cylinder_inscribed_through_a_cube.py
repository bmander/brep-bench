"""A cylinder inscribed through a cube. A cube of side 10 centred on the origin, united with a
cylinder of radius 5 on the x axis from x = −10 to x = 10: the wall touches four of the cube's
faces from inside along their middle lines."""
import usd_brep as sm

HALF = 5          # the cube's half side, and the cylinder's radius
HALF_LENGTH = 10  # of the cylinder


def build():
    cube = sm.create_box((-HALF, -HALF, -HALF), 2 * HALF, 2 * HALF, 2 * HALF)
    cylinder = sm.create_cylinder((0, 0, -HALF_LENGTH), HALF, 2 * HALF_LENGTH)
    # The cylinder's axis is its frame's z and its seam is on the frame's +x side. The frame is
    # placed with z along the world's x and x along (0, 1, 1): the seam on the line y = z, at 45°
    # to the lines of contact.
    s = 0.5 ** 0.5
    sm.transform(cylinder, sm.Axis2Placement((0, 0, 0), (0, s, s), (0, -s, s)))
    return sm.boolean_union(cube, cylinder)
