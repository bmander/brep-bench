+++
title = "A groin vault: two weighted barrel vaults in common"
tags = ["nurbs", "rational", "weights", "boolean", "common", "tangent-crossing", "coplanar"]

[volume]
value = 7632.0
exact = "24²·10 + 8∫₀¹² x(√(225 − x²) − 9) dx = 5760 + 8(882 − 648): above z = 10 the solid's height over (x, y) is 1 + √(225 − m²), m = max(|x|, |y|), so each of the eight triangles 0 ≤ |y| ≤ |x| ≤ 12 (and the same with x, y swapped) integrates in x alone"
tex = '24^2\cdot 10 + 8\int_0^{12} x\left(\sqrt{225 - x^2} - 9\right)dx = 5760 + 8(882 - 648)'

[area]
value = 2256.0
exact = "floor 24² + four sides 4·24·10 + the vaulted top 8∫₀¹² x·15/√(225 − x²) dx = 576 + 960 + 8·90"
tex = '24^2 + 4\cdot 24\cdot 10 + 8\int_0^{12}\frac{15x}{\sqrt{225 - x^2}}\,dx = 576 + 960 + 8\cdot 90'
+++

The common part of two blocks with barrel-vaulted tops, the second the first turned a quarter
turn about the z axis, so that the two vaults cross at right angles: a groin vault.

The first block stands on the square −12 ≤ x ≤ 12, −12 ≤ y ≤ 12, its floor flat at z = 0 and its
four sides upright. Its top is a rational B-spline surface, of degree 1 along x and 2 along y,
non-periodic, with knot vectors (0, 0, 1, 1) along x and (0, 0, 0, 1, 1, 1) along y. Its six
control points are, for x = −12 and again for x = 12, the points (x, −12, 10), (x, 0, 26) and
(x, 12, 10), with weights 1, 3/5 and 1 in that order (so along y each row is one rational
quadratic Bézier arc). With these weights the top is exactly part of a circular cylinder: the
points with y² + (z − 1)² = 15², z ≥ 10, an arch of radius 15 about the line y = 0, z = 1, rising
from z = 10 at the sides y = ±12 to its crown z = 16 along y = 0. (Without the weights it would be
a parabolic vault.) So the sides y = ±12 are rectangles 24 × 10, and the sides x = ±12 are arched.

The second block is the first turned 90° about the z axis: its arch runs along y, rising from
z = 10 at x = ±12 to its crown along x = 0.

Their common part keeps, over every point of the square, the lower of the two vaults. The two
vaults meet along the square's diagonals, in two arches (the groins), each an arc of an ellipse
in the plane x = y or x = −y, running from corner to corner of the square at z = 10 up to
z = 16. The groins cross at the centre, (0, 0, 16), where the two vault surfaces are tangent to
each other. The common part's sides are the four 24 × 10 rectangles,
where each block's flat side lies in the same plane as the other's arched one.
