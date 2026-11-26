+++
title = "A plus-shaped plate grown by 1"
tags = ["offset", "grow", "planar", "concave-edge", "corner", "adversarial"]
origin = "adversarial: offset/l-block-grown with four concave edges instead of one"

[volume]
value = 316.1592854552149
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3) = (716 + 74π)/3, A = 45, P = 36, t = 2, d = 1, k = 2π − 4"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right) = \tfrac{716 + 74\pi}{3},\quad A = 45,\ P = 36,\ t = 2,\ d = 1,\ k = 2\pi - 4'

[area]
value = 293.36281798666926
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²) = 130 + 52π"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right) = 130 + 52\pi'
+++

A plate shaped like a plus sign: its outline in the xy plane is the polygon through (3, 0),
(6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3), a
square 3 × 3 with an arm 3 × 3 on each of its four sides, extruded 2 along +z. Grow it by 1: the
solid of every point within 1 of the plate.

Each face moves 1 outward. Each convex edge becomes a quarter of a cylinder of radius 1 about it,
and each corner where three convex edges meet an eighth of a ball. The four inside corners' edges,
the concave vertical edges through (3, 3), (6, 3), (6, 6) and (3, 6), stay sharp: the two faces
beside each, moved out, meet along a vertical line 1 out from both, such as x = 7, y = 2 beside
the edge through (6, 3). Where such an edge meets the top or the bottom face, the rounds of the
two top or bottom edges beside it meet in a sharp concave crease.

The grown plate's section at any height from z = 0 to z = 2 is the outline grown by 1: its sides
moved out 1, its eight convex corners rounded to radius 1 and its four inside corners sharp, of
area A + Pd + kd², where A = 45 and P = 36 are the outline's area and perimeter and
k = 8·(π/4) − 4·1 counts a quarter disc's worth at each convex corner and takes off the square
the two sides beside an inside corner both cover. Above the top, at height u (0 ≤ u ≤ 1), the
section is the outline grown by √(1 − u²) in the same way, and likewise below the bottom.

**As a set:**

$$\Bigl(\operatorname{poly}\bigl((3,0),(6,0),(6,3),(9,3),(9,6),(6,6),(6,9),(3,9),(3,6),(0,6),(0,3),(3,3)\bigr)\times[0,2]\Bigr) \oplus B_{1}$$
