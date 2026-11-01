+++
title = "An octahedron with every edge chamfered, four chamfers meeting at each corner"
tags = ["chamfer", "corner"]

[volume]
value = 917.3333333333334
exact = "(4/3)a³ − 8(3R − k³) − 6·(4/3)k³, R = ∫ s(a − s) ds over a − k ≤ s ≤ a, a = 10, k = 2 (the chamfers x + y = a − k): in each octant the three wedges beyond the chamfers, less the three regions two of them share, and the six square pyramids beyond the corner planes, apex a − k and base x = a − 2k = 4000/3 − 352 − 64 = 2752/3"
tex = '\tfrac{4}{3}a^3 - 8\left(3\int_{a-k}^{a} s(a - s)\,ds - k^3\right) - 6\cdot\tfrac{4}{3}k^3 = \tfrac{4000}{3} - 352 - 64 = \tfrac{2752}{3},\quad a = 10,\ k = 2'

[area]
value = 478.3802556600424
exact = "8·(√3/4)(4√2)² + 12·(4√2·4) + 6·4² = 64√3 + 192√2 + 96"
tex = '8\cdot\tfrac{\sqrt{3}}{4}\left(4\sqrt{2}\right)^2 + 12\cdot 4\sqrt{2}\cdot 4 + 6\cdot 4^2 = 64\sqrt{3} + 192\sqrt{2} + 96'
+++

The octahedron |x| + |y| + |z| ≤ 10: its six corners on the axes, 10 from the origin, such as
(10, 0, 0), and its eight faces equilateral triangles, such as the one in the plane
x + y + z = 10. Chamfer all twelve of its edges, each with the same distance √6 on both of its
faces, measured across the face square to the edge: the chamfer of the edge from (10, 0, 0) to
(0, 10, 0) is the plane x + y = 8, through the two lines, one on each of its faces, at distance √6
from the edge (on the face x + y + z = 10, the line z = 2). Four chamfers meet at each corner,
and the corner is cut flat by a square.

The solid is the octahedron cut by the twelve chamfer planes, |x| + |y| ≤ 8, |y| + |z| ≤ 8 and
|z| + |x| ≤ 8, and by the six corner planes |x| ≤ 6, |y| ≤ 6 and |z| ≤ 6. It has 26 flat faces.
Each face of the octahedron becomes the equilateral triangle of side 4√2 inset from its edges, for
the face x + y + z = 10 the triangle through (6, 2, 2), (2, 6, 2) and (2, 2, 6). Each chamfer is a
rectangle 4√2 long and 4 wide, between two of those triangles: the chamfer x + y = 8 has corners
(6, 2, 2), (2, 6, 2), (2, 6, −2) and (6, 2, −2). Each corner is the square through the four
triangle corners nearest it: at the corner (10, 0, 0) the square of side 4 in the plane x = 6,
through (6, 2, 2), (6, −2, 2), (6, −2, −2) and (6, 2, −2).

Those four corners lie in one plane by the octahedron's symmetry. Where they do not, as at the
apex of a pyramid on a rectangle, no flat face closes the corner, and only the mitre
(`chamfer/octahedron-all-edges-mitred`) is one solid.
