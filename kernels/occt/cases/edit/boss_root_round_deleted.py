"""A boss's rounded root deleted: the boss stands on the plate with a sharp foot. A plate, the
cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss standing on it, the cylinder
of radius 3 from z = 0 to z = 8. The concave edge where the boss meets the plate, the circle of
radius 3 at z = 0, is rounded to radius 1: the round is a quarter of a torus whose tube has
radius 1 about the circle of radius 4 at z = 1, tangent to the plate's top along the circle of
radius 4 and to the boss's side along the circle at z = 1.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The
plate's top runs in from radius 4 and the boss's side down from z = 1 until they meet in the circle
of radius 3 at z = 0, and the material the round added is gone.

The result is the plate with the boss on it, the edge at the boss's foot sharp."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import cylinder, deleted, on_circle, united


def build():
    solid = united(cylinder((0, 0, -4), 8, 4), cylinder((0, 0, 0), 3, 8))
    solid = rounded(solid, 1, edges_where(solid, on_circle((0, 0, 0), 3)), "the boss's foot")
    # The round at 45° round its tube, whose center line is the circle of radius 4 at z = 1.
    s = math.sqrt(0.5)
    return deleted(solid, [face_through(solid, (0, -(4 - s), 1 - s))])
