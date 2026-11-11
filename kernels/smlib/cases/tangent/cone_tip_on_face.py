"""A cone's tip on a face. A block from (−10, −10, 0) to (10, 10, 10), united with a cone on the z
axis whose apex is at the origin, the middle of the block's bottom face, and whose base is the
disk of radius 5 at z = 15."""
import usd_brep as sm

HALF, THICKNESS = 10, 10
CONE_RADIUS, CONE_HEIGHT = 5, 15


def build():
    block = sm.create_box((-HALF, -HALF, 0), 2 * HALF, 2 * HALF, THICKNESS)
    # the cone's first radius is at its origin: nothing there, the apex
    cone = sm.create_cone((0, 0, 0), 0, CONE_RADIUS, CONE_HEIGHT)
    return sm.boolean_union(block, cone)
