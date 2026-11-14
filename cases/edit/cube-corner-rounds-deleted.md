+++
title = "Three rounds meeting at a cube's corner deleted, with the ball between them"
tags = ["edit", "delete-face", "fillet", "cylinder", "sphere", "corner", "planar", "analytic"]

[volume]
value = 1000.0
exact = "10³ = 1000"
tex = '10^3 = 1000'

[area]
value = 600.0
exact = "6·10² = 600"
tex = '6\cdot 10^2 = 600'
+++

A cube, the box from the origin to (10, 10, 10), with the three edges that meet at its corner (10,
10, 10) rounded to radius 2. Each round is a quarter cylinder about a line 2 inside its edge, and
where the three meet the corner is an eighth of a ball of radius 2 about (8, 8, 8): four faces,
tangent to each other and to the cube's faces.

Then edit the cube directly: delete all four. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
three faces x = 10, y = 10 and z = 10 run on until they meet in the three edges again, and in the
corner (10, 10, 10).

The result is the cube.

**As a set:**

$$[0,10]^3$$
