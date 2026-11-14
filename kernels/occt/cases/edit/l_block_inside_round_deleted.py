"""An L-block's rounded inside corner deleted: the corner is sharp again. An L-shaped block: the
outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in the xy plane, extruded 8
along +z. Its inside corner, the concave edge along the line x = 5, y = 5, is rounded to radius
2: the round is a quarter cylinder whose axis is the line x = 7, y = 7, tangent to the face x = 5
along y = 7 and to the face y = 5 along x = 7. It fills the corner.

Then edit the block directly: delete the round. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
face x = 5 runs back from y = 7 and the face y = 5 from x = 7 until they meet in the edge x = 5, y =
5, and the material the round added is gone.

The result is the L-shaped block with its inside corner sharp."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import deleted, extruded, middle_at


def build():
    outline = [(0, 0, 0), (20, 0, 0), (20, 5, 0), (5, 5, 0), (5, 15, 0), (0, 15, 0)]
    block = extruded(outline, (0, 0, 8))
    block = rounded(block, 2, edges_where(block, middle_at((5, 5, 4))), "the inside corner")
    # The round at its middle: 2 from its axis, the line x = 7, y = 7, at 45°.
    s = math.sqrt(2)
    return deleted(block, [face_through(block, (7 - s, 7 - s, 4))])
