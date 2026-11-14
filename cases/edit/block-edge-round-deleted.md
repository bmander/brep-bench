+++
title = "A block's rounded edge deleted: the top and the side meet again"
tags = ["edit", "delete-face", "fillet", "cylinder", "planar", "analytic"]

[volume]
value = 2000.0
exact = "20·10·10 = 2000"
tex = '20\cdot 10\cdot 10 = 2000'

[area]
value = 1000.0
exact = "2(20·10 + 20·10 + 10·10) = 1000"
tex = '2(20\cdot 10 + 20\cdot 10 + 10\cdot 10) = 1000'
+++

A block, the box from the origin to (20, 10, 10), with the edge where its top, z = 10, meets its
side x = 20 rounded to radius 2: the round is a quarter cylinder whose axis is the line x = 18, z =
8, tangent to the top along x = 18 and to the side along z = 8.

Then edit the block directly: delete the round. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
top runs on from x = 18 and the side from z = 8 until they meet in the edge x = 20, z = 10, and the
two ends, y = 0 and y = 10, get their corners back.

The result is the box from the origin to (20, 10, 10).

**As a set:**

$$[0,20]\times[0,10]\times[0,10]$$
