+++
title = "A U-shaped plate grown by 1"
tags = ["offset", "grow", "planar", "concave-edge", "slot", "corner", "adversarial"]
origin = "adversarial: offset/l-block-grown with a slot, two concave edges facing each other"

[volume]
value = 432.4395202874373
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3) = 1024/3 + 29π, A = 63, P = 48, t = 2, d = 1, k = 3π/2 − 2"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right) = \tfrac{1024}{3} + 29\pi,\quad A = 63,\ P = 48,\ t = 2,\ d = 1,\ k = \tfrac{3\pi}{2} - 2'

[area]
value = 394.4955592153876
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²) = 206 + 60π"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right) = 206 + 60\pi'
+++

A U-shaped plate: its outline in the xy plane is the polygon through (0, 0), (9, 0), (9, 9),
(6, 9), (6, 3), (3, 3), (3, 9) and (0, 9), a square 9 × 9 with a slot 3 wide cut 6 deep into it
from the side y = 9, extruded 2 along +z. Grow it by 1: the solid of every point within 1 of the
plate.

Each face moves 1 outward. Each convex edge becomes a quarter of a cylinder of radius 1 about it,
and each corner where three convex edges meet an eighth of a ball. The slot's walls move 1 into
it, leaving it 1 wide, from x = 4 to x = 5, and its floor moves up to y = 4. The slot's two
inside corners, the concave vertical edges through (3, 3) and (6, 3), stay sharp: the faces
beside each, moved, meet along the vertical lines x = 4, y = 4 and x = 5, y = 4. Where such an
edge meets the top or the bottom face, the rounds of the two top or bottom edges beside it meet in
a sharp concave crease.

The grown plate's section at any height from z = 0 to z = 2 is the outline grown by 1: its sides
moved out 1, its six convex corners rounded to radius 1 and its two inside corners sharp, of area
A + Pd + kd², where A = 63 and P = 48 are the outline's area and perimeter and
k = 6·(π/4) − 2·1 counts a quarter disc's worth at each convex corner and takes off the square
that the two sides beside an inside corner both cover. Above the top, at height u (0 ≤ u ≤ 1),
the section is the outline grown by √(1 − u²) in the same way, and likewise below the bottom.

**As a set:**

$$\Bigl(\operatorname{poly}\bigl((0,0),(9,0),(9,9),(6,9),(6,3),(3,3),(3,9),(0,9)\bigr)\times[0,2]\Bigr) \oplus B_{1}$$
