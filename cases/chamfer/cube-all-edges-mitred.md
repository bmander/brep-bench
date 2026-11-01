+++
title = "A cube with every edge chamfered, its corners mitred"
tags = ["chamfer", "corner", "mitre"]

[volume]
value = 808.0
exact = "a³ − 12·(d²/2)(a − 2d) − 8·(3d³/4) = a³ − 6ad² + 6d³, a = 10, d = 2: twelve prisms along the edges, and in each corner's cube of side d (u, v, w its distances from the three faces) all but the points with u + v, v + w and w + u each at least d, which fill d³/4 = 808"
tex = 'a^3 - 12\cdot\tfrac{d^2}{2}(a - 2d) - 8\cdot\tfrac{3d^3}{4} = a^3 - 6ad^2 + 6d^3 = 808,\quad a = 10,\ d = 2'

[area]
value = 453.58787847867995
exact = "6(a − 2d)² + 12·((a − 2d)·d√2 + 2·(d√2)(d/2)/2) = 216 + 168√2: each chamfer the rectangle of the cornered case with a triangle of base d√2 and height d/2 at each end"
tex = '6(a - 2d)^2 + 12\left((a - 2d)\,d\sqrt{2} + 2\cdot\tfrac{1}{2}\,d\sqrt{2}\cdot\tfrac{d}{2}\right) = 216 + 168\sqrt{2},\quad a = 10,\ d = 2'
+++

A cube of side 10 between the origin and (10, 10, 10). Chamfer all twelve of its edges, each with
the same distance 2 on both of its faces: an edge's chamfer is the plane through the two lines,
one on each of its faces, at distance 2 from the edge. No face closes a corner: each chamfer runs
on until it meets the others, so the three at each corner meet in a point, the corner mitred.

The solid is the cube cut by the twelve chamfer planes, x + y ≤ 18 for the edge from (10, 10, 0)
to (10, 10, 10) and the others likewise; `chamfer/cube-all-edges` is this solid cut again by a
plane at each corner. It has 18 flat faces. Each face of the cube becomes the 6 × 6 square inset 2
from its edges, for the top face the square from (2, 2, 10) to (8, 8, 10), as in the cornered
case. Each chamfer is a hexagon, a rectangle 6 long and 2√2 wide with a triangle at each end: the
chamfer of the edge from (10, 10, 0) to (10, 10, 10) has corners (9, 9, 1), (10, 8, 2),
(10, 8, 8), (9, 9, 9), (8, 10, 8) and (8, 10, 2). The three chamfers at the corner (10, 10, 10)
meet at the point (9, 9, 9), each pair along a line from it to a square's corner, such as from
(9, 9, 9) to (8, 8, 10).

It is what chamfering the edges one after another leaves, each edge its own chamfer, ending on
the faces at its ends, the chamfers made before it among them: build it so, with the kernel's
chamfer, in any order.
