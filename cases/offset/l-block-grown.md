+++
title = "An L-shaped plate grown by 1"
tags = ["offset", "grow", "planar", "concave-edge", "corner"]

[volume]
value = 2197.27432383419
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3), A = 150, P = 70, t = 8, d = 1, k = 5π/4 − 1"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right),\quad A = 150,\ P = 70,\ t = 8,\ d = 1,\ k = \tfrac{5\pi}{4} - 1'

[area]
value = 1138.4513020910304
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²)"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right)'
+++

The L-shaped plate whose outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5),
(5, 5), (5, 15) and (0, 15), extruded 8 along +z. Grow it by 1: the solid of every point within 1
of the plate.

Each face moves 1 outward. Each convex edge becomes a quarter of a cylinder of radius 1 about it,
and each corner where three convex edges meet an eighth of a ball. The inside corner's edge, the
concave one along (5, 5), stays sharp: the two faces beside it, moved out, meet along the line
x = 6, y = 6. Where that edge meets the top and bottom faces, the rounds of the top or bottom
edges beside it meet in a sharp concave crease.

The grown plate's section at any height from z = 0 to z = 8 is the outline grown by 1 (its sides
moved out 1, its five convex corners rounded to radius 1, its inside corner sharp), of area
A + Pd + kd², where A and P are the outline's area and perimeter.
