+++
title = "An L-block's rounded inside corner deleted: the corner is sharp again"
tags = ["edit", "delete-face", "fillet", "cylinder", "planar", "concave-edge", "analytic"]

[volume]
value = 1200.0
exact = "150·8 = 1200: the outline's area, 150, times the height"
tex = '150\cdot 8 = 1200'

[area]
value = 860.0
exact = "2·150 + 70·8 = 860: the two ends, and the outline's perimeter, 70, times the height"
tex = '2\cdot 150 + 70\cdot 8 = 860'
+++

An L-shaped block: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in the
xy plane, extruded 8 along +z. Its inside corner, the concave edge along the line x = 5, y = 5, is
rounded to radius 2: the round is a quarter cylinder whose axis is the line x = 7, y = 7, tangent to
the face x = 5 along y = 7 and to the face y = 5 along x = 7. It fills the corner.

Then edit the block directly: delete the round. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
face x = 5 runs back from y = 7 and the face y = 5 from x = 7 until they meet in the edge x = 5, y =
5, and the material the round added is gone.

The result is the L-shaped block with its inside corner sharp.

**As a set:**

$$\bigl([0,20]\times[0,5] \cup [0,5]\times[0,15]\bigr)\times[0,8]$$
