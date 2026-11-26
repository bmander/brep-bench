+++
title = "A plate with a square hole grown by 1"
tags = ["offset", "grow", "planar", "concave-edge", "hole", "corner", "adversarial"]
origin = "adversarial: offset/drilled-plate-grown with a square hole, whose four corners are concave edges"

[volume]
value = 533.1032364791469
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3) = (1304 + 94π)/3, A = 10² − 4² = 84, P = 4·10 + 4·4 = 56, t = 2, d = 1, k = π − 4"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right) = \tfrac{1304 + 94\pi}{3},\quad A = 84,\ P = 56,\ t = 2,\ d = 1,\ k = \pi - 4'

[area]
value = 449.06192982974676
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²) = 248 + 64π"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right) = 248 + 64\pi'
+++

A square plate from (0, 0, 0) to (10, 10, 2) with a square hole through it, the square from
(3, 3) to (7, 7), its sides parallel to the plate's. Grow it by 1: the solid of every point
within 1 of the plate.

Each face moves 1 outward. Each convex edge, the plate's outer edges and the hole's rims on the
top and the bottom, becomes a quarter of a cylinder of radius 1 about it, and each outer corner
an eighth of a ball. The hole's four walls move 1 into the hole, which is left the square from
(4, 4) to (6, 6) through the grown plate's middle. The hole's four vertical edges are concave and
stay sharp: the two walls beside each, moved, meet along a vertical line, such as x = 4, y = 4.
Where such an edge meets the top or the bottom, the rounds of the two hole rims beside it meet in
a sharp concave crease.

The grown plate's section at any height from z = 0 to z = 2 is the plate's section grown by 1:
the outer square's sides moved out 1 and its corners rounded to radius 1, the hole's sides moved
in 1 and its corners sharp, of area A + Pd + kd², where A = 84 and P = 56 are the section's area
and the length of its boundary, both squares, and k = 4·(π/4) − 4·1 counts a quarter disc's
worth at each outer corner and takes off the square that the two walls beside a hole corner both
cover. Above the top, at height u (0 ≤ u ≤ 1), the section is the plate's section grown by
√(1 − u²) in the same way, and likewise below the bottom.

**As a set:**

$$\Bigl(\bigl([0,10]^2 \setminus [3,7]^2\bigr)\times[0,2]\Bigr) \oplus B_{1}$$
