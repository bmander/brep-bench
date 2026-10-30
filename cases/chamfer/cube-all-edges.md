+++
title = "A cube with every edge chamfered"
tags = ["chamfer", "corner"]

[volume]
value = 802.6666666666666
exact = "a³ − 12·(d²/2)(a − 2d) − 8·(d³ − d³/6) = a³ − 6ad² + (16/3)d³, a = 10, d = 2: twelve prisms along the edges, and in each corner's cube of side d all but the tetrahedron (d, d, d), (d, d, 0), (d, 0, d), (0, d, d) cut away = 2408/3"
tex = 'a^3 - 12\cdot\tfrac{d^2}{2}(a - 2d) - 8\left(d^3 - \tfrac{d^3}{6}\right) = a^3 - 6ad^2 + \tfrac{16}{3}d^3 = \tfrac{2408}{3},\quad a = 10,\ d = 2'

[area]
value = 447.3595659028278
exact = "6(a − 2d)² + 12(a − 2d)·d√2 + 8·(√3/4)(d√2)² = 216 + 144√2 + 16√3"
tex = '6(a - 2d)^2 + 12(a - 2d)\,d\sqrt{2} + 8\cdot\tfrac{\sqrt{3}}{4}\left(d\sqrt{2}\right)^2 = 216 + 144\sqrt{2} + 16\sqrt{3},\quad a = 10,\ d = 2'
+++

A cube of side 10 between the origin and (10, 10, 10). Chamfer all twelve of its edges, each with
the same distance 2 on both of its faces: an edge's chamfer is the plane through the two lines,
one on each of its faces, at distance 2 from the edge. Three chamfers meet at each corner, and
the corner is cut flat by a triangle.

The result has 26 flat faces. Each face of the cube becomes the 6 × 6 square inset 2 from its
edges, for the top face the square from (2, 2, 10) to (8, 8, 10). Each chamfer is a rectangle 6
long and 2√2 wide, between two of those squares: the chamfer of the edge from (10, 10, 0) to
(10, 10, 10) is the rectangle with corners (10, 8, 2), (8, 10, 2), (8, 10, 8) and (10, 8, 8).
Each corner is the equilateral triangle through the three square corners nearest it: at the
corner (10, 10, 10) the triangle through (8, 8, 10), (8, 10, 8) and (10, 8, 8), in the plane
x + y + z = 26.
