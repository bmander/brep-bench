+++
title = "An octahedron with every edge chamfered, its corners mitred"
tags = ["chamfer", "corner", "mitre"]

[volume]
value = 981.3333333333334
exact = "(4/3)a³ − 8(3R − k³), R = ∫ s(a − s) ds over a − k ≤ s ≤ a, a = 10, k = 2 (the chamfers x + y = a − k): in each octant the three wedges beyond the chamfers, each R = 52/3, less the three regions two of them share, each k³/3, none shared by all three = 4000/3 − 352 = 2944/3"
tex = '\tfrac{4}{3}a^3 - 8\left(3\int_{a-k}^{a} s(a - s)\,ds - k^3\right) = \tfrac{4000}{3} - 352 = \tfrac{2944}{3},\quad a = 10,\ k = 2'

[area]
value = 518.1447576478596
exact = "8·(√3/4)(4√2)² + 12·(4√2·4 + 2·(4·2√2)/2) = 64√3 + 288√2: each chamfer the rectangle of the cornered case with a triangle of base 4 and height 2√2 at each end"
tex = '8\cdot\tfrac{\sqrt{3}}{4}\left(4\sqrt{2}\right)^2 + 12\left(4\sqrt{2}\cdot 4 + 2\cdot\tfrac{1}{2}\cdot 4\cdot 2\sqrt{2}\right) = 64\sqrt{3} + 288\sqrt{2}'
+++

The octahedron |x| + |y| + |z| ≤ 10: its six corners on the axes, 10 from the origin, such as
(10, 0, 0), and its eight faces equilateral triangles, such as the one in the plane
x + y + z = 10. Chamfer all twelve of its edges, each with the same distance √6 on both of its
faces, measured across the face square to the edge: the chamfer of the edge from (10, 0, 0) to
(0, 10, 0) is the plane x + y = 8, through the two lines, one on each of its faces, at distance √6
from the edge (on the face x + y + z = 10, the line z = 2). No face closes a corner: each chamfer
runs on until it meets the others, so the four at each corner meet in a point, the corner mitred.

The solid is the octahedron cut by the twelve chamfer planes, |x| + |y| ≤ 8, |y| + |z| ≤ 8 and
|z| + |x| ≤ 8; `chamfer/octahedron-all-edges` is this solid cut again by a plane at each corner.
It has 20 flat faces. Each face of the octahedron becomes the equilateral triangle of side 4√2
inset from its edges, for the face x + y + z = 10 the triangle through (6, 2, 2), (2, 6, 2) and
(2, 2, 6). Each chamfer is a hexagon, a rectangle 4√2 long and 4 wide with a triangle at each end:
the chamfer x + y = 8 has corners (8, 0, 0), (6, 2, 2), (2, 6, 2), (0, 8, 0), (2, 6, −2) and
(6, 2, −2). The four chamfers at the corner (10, 0, 0) meet at the point (8, 0, 0).

It is what chamfering the edges one after another leaves, each edge its own chamfer, ending on
the faces at its ends, the chamfers made before it among them: build it so, with the kernel's
chamfer, in any order.
