+++
title = "A weighted barrel vault cut by the plane along its crown"
tags = ["nurbs", "rational", "weights", "boolean", "cut", "tangent-line"]

[volume]
value = 8175.394177208706
exact = "the cut removes nothing: 24·∫₋₁₂¹² (1 + √(225 − y²)) dy = 24(132 + 225·asin(4/5)) = 3168 + 5400·asin(4/5)"
tex = '24\int_{-12}^{12}\left(1 + \sqrt{225 - y^2}\right)dy = 24\left(132 + 225\arcsin\tfrac{4}{5}\right) = 3168 + 5400\arcsin\tfrac{4}{5}'

[area]
value = 2404.9354050618863
exact = "floor 576 + flat sides 2·24·10 + arched sides 2(132 + 225·asin(4/5)) + vault 24·15·2·asin(4/5) = 1320 + 1170·asin(4/5)"
tex = '576 + 2\cdot 24\cdot 10 + 2\left(132 + 225\arcsin\tfrac{4}{5}\right) + 24\cdot 15\cdot 2\arcsin\tfrac{4}{5} = 1320 + 1170\arcsin\tfrac{4}{5}'
+++

A block with a barrel-vaulted top, from which everything above the plane z = 16 is cut away. That
plane touches the vault along its crown, so the cut removes nothing and the solid is the whole
block; the cut is the case.

The block stands on the square −12 ≤ x ≤ 12, −12 ≤ y ≤ 12, its floor flat at z = 0 and its four
sides upright. Its top is a rational B-spline surface, of degree 1 along x and 2 along y,
non-periodic, with knot vectors (0, 0, 1, 1) along x and (0, 0, 0, 1, 1, 1) along y. Its six
control points are, for x = −12 and again for x = 12, the points (x, −12, 10), (x, 0, 26) and
(x, 12, 10), with weights 1, 3/5 and 1 in that order (so along y each row is one rational
quadratic Bézier arc). With these weights the top is exactly part of a circular cylinder: the
points with y² + (z − 1)² = 15², z ≥ 10, an arch of radius 15 about the line y = 0, z = 1, rising
from z = 10 at the sides y = ±12 to its crown z = 16 along the line y = 0. So the sides y = ±12
are rectangles 24 × 10, and the sides x = ±12 are arched.

Remove from the block everything with z > 16, for example by cutting it with a box whose floor is
the plane z = 16 and which reaches past the block on every other side. The box's floor touches
the vault along the whole crown line, from (−12, 0, 16) to (12, 0, 16), tangent to it there, and
meets the block nowhere else.
