"""A cylinder's beveled rim deleted: the top and the side meet again. A cylinder of radius 6 on the
z axis from z = 0 to z = 10, with its top rim beveled 2 back on each face: the cylinder from z =
0 to z = 8 with a cone frustum on it, from radius 6 at z = 8 to radius 4 at z = 10. The bevel is
the frustum's side.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on from radius 4 and the side from z = 8 until they meet in the circle of radius 6 at z = 10.

The result is the cylinder of radius 6 from z = 0 to z = 10."""
from bench import face_through
from edit import cylinder, deleted, frustum, united


def build():
    beveled = united(cylinder((0, 0, 0), 6, 8), frustum((0, 0, 8), 6, 4, 2))
    return deleted(beveled, [face_through(beveled, (0, -5, 9))])
