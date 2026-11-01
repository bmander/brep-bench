+++
title = "A creased sheet thickened outside its crease, the corner kept sharp"
tags = ["thicken", "sheet", "planar", "crease", "sharp-edge"]

[volume]
value = 168
exact = "w((l + t)t + lt), l = 10, t = 1, w = 8"
tex = 'w\left((l + t)\,t + lt\right),\quad l = 10,\ t = 1,\ w = 8'

[area]
value = 394
exact = "2((l + t)t + lt) + w(4l + 4t)"
tex = '2\left((l + t)\,t + lt\right) + w\,(4l + 4t)'
+++

A sheet of two flat faces meeting at a right angle along a crease: the rectangle in the xy plane
from (0, 0, 0) to (10, 8, 0), the floor, and the rectangle in the yz plane from (0, 0, 0) to
(0, 8, 10), the wall. They share the edge from (0, 0, 0) to (0, 8, 0).

Thicken it by 1 to the outside of the crease, below the floor and behind the wall (to −z and −x),
with the corner kept sharp: each face moves 1 along its normal, and the moved faces, which part at
the crease, are extended until they meet, so that no round is added.

The thickened sheet is an angle 1 thick: its section in any plane y = const is the L through
(−1, −1), (10, −1), (10, 0), (0, 0), (0, 10) and (−1, 10), each given as (x, z), extruded from
y = 0 to y = 8.
