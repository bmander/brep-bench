"""A box with every edge rounded, all twenty rounds deleted. A box from the origin to (20, 16, 12)
with every edge rounded to radius 2: twelve quarter cylinders, one along each edge about a line 2
inside it, and eight eighths of a ball of radius 2, one at each corner about the point 2 inside
it along each axis. Its six flat faces are left as rectangles 4 shorter each way.

Then edit the box directly: delete all twenty rounds, the twelve cylinders and the eight balls.
Deleting a face takes it out of the solid and closes the gap it leaves with the faces round it, each
run on along its own surface until they meet. Nothing is left beside a flat face but other flat
faces, so the six run on until they meet in the box's twelve edges and eight corners.

The result is the box from the origin to (20, 16, 12)."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import box, deleted


def build():
    a, b, c = 20, 16, 12
    block = box((0, 0, 0), (a, b, c))
    block = rounded(block, 2, edges_where(block, lambda middle: True), "the box's edges")
    # A point of each round at 45° about its axis, and of each ball along the diagonal from its
    # center: near(u, length, d) is that far in from the face at 0 (u = 0) or at `length` (u = 1).
    near = lambda u, length, d: 2 - d if u == 0 else length - 2 + d
    s, t = math.sqrt(2), 2 / math.sqrt(3)
    blends = []
    for u in (0, 1):
        for v in (0, 1):
            blends += [(near(u, a, s), near(v, b, s), c / 2), (a / 2, near(u, b, s), near(v, c, s)),
                       (near(u, a, s), b / 2, near(v, c, s))]
            blends += [(near(u, a, t), near(v, b, t), near(w, c, t)) for w in (0, 1)]
    return deleted(block, [face_through(block, point) for point in blends])
