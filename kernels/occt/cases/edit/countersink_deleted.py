"""A hole's countersink deleted: the hole runs straight up to the top. A block, the box from the
origin to (20, 10, 10), drilled right through along z by a hole of radius 2 whose axis is the
vertical line through (10, 5), and countersunk at the top: the cone about the same axis from
radius 2 at z = 8 to radius 4 at z = 10 is cut away, so the hole's mouth in the top is a circle
of radius 4 and the hole's wall stops at z = 8.

Then edit the block directly: delete the countersink, the cone's face. Deleting a face takes it out
of the solid and closes the gap it leaves with the faces round it, each run on along its own surface
until they meet. The hole's wall runs on up from z = 8 and the top closes in from radius 4 until
they meet in the circle of radius 2 at z = 10.

The result is the box from the origin to (20, 10, 10), drilled right through along z by a hole of
radius 2 through (10, 5)."""
from bench import face_through
from edit import box, deleted, drilled, frustum, less


def build():
    block = drilled(box((0, 0, 0), (20, 10, 10)), 10, 5, 2, -1, 11)
    # The countersink's cone, run on past the top to z = 11, where its radius is 5.
    countersunk = less(block, frustum((10, 5, 8), 2, 5, 3))
    return deleted(countersunk, [face_through(countersunk, (13, 5, 9))])
