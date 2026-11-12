"""A block's beveled edge deleted: the top and the side meet again. A block, the box from the origin
to (20, 10, 10), with the edge where its top, z = 10, meets its side x = 20 beveled 3 back on
each face: the bevel is the plane through the lines x = 17, z = 10 and x = 20, z = 7. The beveled
block is the pentagon through (0, 0), (20, 0), (20, 7), (17, 10) and (0, 10) in the plane y = 0,
its corners given as (x, z), extruded 10 along +y.

Then edit the block directly: delete the bevel. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
top runs on from x = 17 and the side from z = 7 until they meet in the edge x = 20, z = 10.

The result is the box from the origin to (20, 10, 10)."""
from bench import face_through
from edit import deleted, extruded


def build():
    beveled = extruded([(0, 0, 0), (20, 0, 0), (20, 0, 7), (17, 0, 10), (0, 0, 10)], (0, 10, 0))
    return deleted(beveled, [face_through(beveled, (18.5, 5, 8.5))])
