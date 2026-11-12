"""A block's rounded edge deleted: the top and the side meet again. A block, the box from the origin
to (20, 10, 10), with the edge where its top, z = 10, meets its side x = 20 rounded to radius 2:
the round is a quarter cylinder whose axis is the line x = 18, z = 8, tangent to the top along x
= 18 and to the side along z = 8.

Then edit the block directly: delete the round. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
top runs on from x = 18 and the side from z = 8 until they meet in the edge x = 20, z = 10, and the
two ends, y = 0 and y = 10, get their corners back.

The result is the box from the origin to (20, 10, 10)."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import box, deleted, middle_at


def build():
    block = box((0, 0, 0), (20, 10, 10))
    block = rounded(block, 2, edges_where(block, middle_at((20, 5, 10))), "the block's edge")
    # The round at its middle: 2 from its axis, the line x = 18, z = 8, at 45°.
    s = math.sqrt(2)
    return deleted(block, [face_through(block, (18 + s, 5, 8 + s))])
