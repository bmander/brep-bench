+++
title = "A box with every edge rounded, all twenty rounds deleted"
tags = ["edit", "delete-face", "fillet", "cylinder", "sphere", "corner", "planar", "analytic"]

[volume]
value = 3840.0
exact = "20·16·12 = 3840"
tex = '20\cdot 16\cdot 12 = 3840'

[area]
value = 1504.0
exact = "2(20·16 + 20·12 + 16·12) = 1504"
tex = '2(20\cdot 16 + 20\cdot 12 + 16\cdot 12) = 1504'
+++

A box from the origin to (20, 16, 12) with every edge rounded to radius 2: twelve quarter cylinders,
one along each edge about a line 2 inside it, and eight eighths of a ball of radius 2, one at each
corner about the point 2 inside it along each axis. Its six flat faces are left as rectangles 4
shorter each way.

Then edit the box directly: delete all twenty rounds, the twelve cylinders and the eight balls.
Deleting a face takes it out of the solid and closes the gap it leaves with the faces round it, each
run on along its own surface until they meet. Nothing is left beside a flat face but other flat
faces, so the six run on until they meet in the box's twelve edges and eight corners.

The result is the box from the origin to (20, 16, 12).

**As a set:**

$$[0,20]\times[0,16]\times[0,12]$$
