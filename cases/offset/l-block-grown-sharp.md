+++
title = "An L-shaped plate grown by 1, its edges kept sharp"
tags = ["offset", "grow", "planar", "sharp-edge"]

[volume]
value = 2240
exact = "(t + 2d)·224, t = 8, d = 1; 224 is the area of the outline moved 1 outward"
tex = '(t + 2d)\cdot 224,\quad t = 8,\ d = 1'

[area]
value = 1228
exact = "2·224 + (t + 2d)·78; 78 is the moved outline's perimeter"
tex = '2\cdot 224 + (t + 2d)\cdot 78'
+++

The L-shaped plate whose outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 5),
(5, 5), (5, 15) and (0, 15), extruded 8 along +z. Grow it by 1 with its edges kept sharp: move
each face 1 outward along its normal, and extend the moved faces until they meet, so that no round
is added and every edge stays where its two faces meet.

The grown plate is the outline through (−1, −1), (21, −1), (21, 6), (6, 6), (6, 16) and (−1, 16),
extruded from z = −1 to z = 9.
