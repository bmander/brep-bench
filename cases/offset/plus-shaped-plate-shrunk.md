+++
title = "A plus-shaped plate shrunk by 1"
tags = ["offset", "shrink", "planar", "concave-edge", "round", "adversarial"]
origin = "adversarial: offset/l-block-shrunk with four concave edges, each rounded by the shrink"

[volume]
value = 27.716814692820414
exact = "(t − 2d)(A − Pd + (8 − π)d²) = 34 − 2π, A = 45, P = 36, t = 4, d = 1"
tex = '(t - 2d)\left(A - Pd + (8 - \pi)d^2\right) = 34 - 2\pi,\quad A = 45,\ P = 36,\ t = 4,\ d = 1'

[area]
value = 80.28318530717959
exact = "2(A − Pd + (8 − π)d²) + (t − 2d)(P − 16d + 2πd) = 74 + 2π"
tex = '2\left(A - Pd + (8 - \pi)d^2\right) + (t - 2d)\left(P - 16d + 2\pi d\right) = 74 + 2\pi'
+++

A plate shaped like a plus sign, as in `offset/plus-shaped-plate-grown` but 4 thick: its outline
in the xy plane is the polygon through (3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9),
(3, 9), (3, 6), (0, 6), (0, 3) and (3, 3), extruded 4 along +z. Shrink it by 1: keep every point of
it at least 1 from its boundary.

The top and the bottom move in to z = 3 and z = 1, and each side moves 1 into the material, so
each arm, 3 wide, is left 1 wide. The eight convex vertical edges and the edges round the top and
the bottom stay sharp. Each of the four concave vertical edges is rounded by the shrink: the
material left there is bounded by the cylinder of radius 1 about the edge, such as the one about
the vertical line through (6, 3), between the two moved sides, which it meets tangentially.

The section at any height from z = 1 to z = 3 is the outline shrunk by 1: of area
A − Pd + (8·tan 45° − 4·π/4)d², its eight convex corners still square and its four inside corners
rounded to radius 1.

**As a set:**

$$\Bigl(\operatorname{poly}\bigl((3,0),(6,0),(6,3),(9,3),(9,6),(6,6),(6,9),(3,9),(3,6),(0,6),(0,3),(3,3)\bigr)\times[0,4]\Bigr) \ominus B_{1}$$
