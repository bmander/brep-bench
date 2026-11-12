"""Three rounds meeting at a cube's corner deleted, with the ball between them. A cube, the box from
the origin to (10, 10, 10), with the three edges that meet at its corner (10, 10, 10) rounded to
radius 2. Each round is a quarter cylinder about a line 2 inside its edge, and where the three
meet the corner is an eighth of a ball of radius 2 about (8, 8, 8): four faces, tangent to each
other and to the cube's faces.

Then edit the cube directly: delete all four. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
three faces x = 10, y = 10 and z = 10 run on until they meet in the three edges again, and in the
corner (10, 10, 10).

The result is the cube."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import box, deleted, middle_at


def build():
    cube = box((0, 0, 0), (10, 10, 10))
    edges = [edge for middle in [(5, 10, 10), (10, 5, 10), (10, 10, 5)]
             for edge in edges_where(cube, middle_at(middle))]
    cube = rounded(cube, 2, edges, "the three edges at the corner")
    # Each round at 45° about its axis, and the ball along the diagonal from its center (8, 8, 8).
    s, t = 8 + math.sqrt(2), 8 + 2 / math.sqrt(3)
    blends = [(4, s, s), (s, 4, s), (s, s, 4), (t, t, t)]
    return deleted(cube, [face_through(cube, point) for point in blends])
