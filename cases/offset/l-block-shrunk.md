+++
title = "An L-shaped plate shrunk by 1"
tags = ["offset", "shrink", "planar", "concave-edge"]

[volume]
value = 505.2876110196153
exact = "(t − 2d)(A − Pd + kd²), A = 150, P = 70, t = 8, d = 1, k = 5 − π/4"
tex = '(t - 2d)(A - Pd + kd^2),\quad A = 150,\ P = 70,\ t = 8,\ d = 1,\ k = 5 - \tfrac{\pi}{4}'

[area]
value = 537.8539816339745
exact = "2(A − Pd + kd²) + (t − 2d)(P − 2kd)"
tex = '2(A - Pd + kd^2) + (t - 2d)(P - 2kd)'
+++

The L-shaped plate whose outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5),
(5, 5), (5, 15) and (0, 15), extruded 8 along +z. Shrink it by 1: the solid of every point at
least 1 from the plate's outside.

Each face moves 1 inward and every convex edge stays sharp. The inside corner's edge, the concave
one along (5, 5), becomes a round: a quarter of a cylinder of radius 1 about that edge, concave,
joining the two faces beside it, which have moved in to x = 4 and y = 4.

So the shrunk plate runs from z = 1 to z = 7, and its section is the polygon through (1, 1),
(19, 1), (19, 4), (5, 4), (4, 5), (4, 14) and (1, 14) with its edge from (5, 4) to (4, 5) replaced
by the quarter circle of radius 1 about (5, 5).
