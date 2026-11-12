"""A cylinder's rounded rim deleted: the top and the side meet again. A cylinder of radius 6 on the
z axis from z = 0 to z = 10, with its top rim rounded to radius 2: the round is a quarter of a
torus whose tube has radius 2 about the circle of radius 4 at z = 8, tangent to the top along the
circle of radius 4 and to the side along the circle at z = 8.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on from radius 4 and the side from z = 8 until they meet in the circle of radius 6 at z = 10.

The result is the cylinder of radius 6 from z = 0 to z = 10."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import cylinder, deleted, on_circle


def build():
    solid = cylinder((0, 0, 0), 6, 10)
    solid = rounded(solid, 2, edges_where(solid, on_circle((0, 0, 10), 6)), "the cylinder's rim")
    # The round at 45° round its tube, whose center line is the circle of radius 4 at z = 8.
    s = math.sqrt(2)
    return deleted(solid, [face_through(solid, (0, -(4 + s), 8 + s))])
