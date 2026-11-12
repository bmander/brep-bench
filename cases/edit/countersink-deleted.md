+++
title = "A hole's countersink deleted: the hole runs straight up to the top"
tags = ["edit", "delete-face", "cone", "cylinder", "planar", "hole", "analytic"]

[volume]
value = 1874.3362938564083
exact = "20·10·10 − π·2²·10 = 2000 − 40π"
tex = '20\cdot 10\cdot 10 - \pi\cdot 2^2\cdot 10 = 2000 - 40\pi'

[area]
value = 1100.5309649148734
exact = "1000 − 2π·2² + 2π·2·10 = 1000 + 32π: the block's surface less the hole's two ends, and the hole's wall"
tex = '1000 - 2\pi\cdot 2^2 + 2\pi\cdot 2\cdot 10 = 1000 + 32\pi'
+++

A block, the box from the origin to (20, 10, 10), drilled right through along z by a hole of radius
2 whose axis is the vertical line through (10, 5), and countersunk at the top: the cone about the
same axis from radius 2 at z = 8 to radius 4 at z = 10 is cut away, so the hole's mouth in the top
is a circle of radius 4 and the hole's wall stops at z = 8.

Then edit the block directly: delete the countersink, the cone's face. Deleting a face takes it out
of the solid and closes the gap it leaves with the faces round it, each run on along its own surface
until they meet. The hole's wall runs on up from z = 8 and the top closes in from radius 4 until
they meet in the circle of radius 2 at z = 10.

The result is the box from the origin to (20, 10, 10), drilled right through along z by a hole of
radius 2 through (10, 5).

**As a set:**

$$\bigl([0,20]\times[0,10]\times[0,10]\bigr) \setminus \{\,(x-10)^2 + (y-5)^2 < 2^2\,\}$$
