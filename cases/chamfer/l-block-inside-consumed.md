+++
title = "An L-block whose inside corner is chamfered away"
tags = ["chamfer", "concave", "vanishing-face"]

[volume]
value = 755.0
exact = "10·(51 + 7²/2): the L's section of area 51 and the triangle the chamfer fills"
tex = '10\left(51 + \tfrac{7^2}{2}\right)'

[area]
value = 509.99494936611666
exact = "2·75.5 + 10·(10 + 3 + 7√2 + 3 + 10) = 411 + 70√2"
tex = '2\cdot 75.5 + 10\left(10 + 3 + 7\sqrt{2} + 3 + 10\right) = 411 + 70\sqrt{2}'
+++

An L-shaped block. Its section in the xz plane is the polygon through (0, 0, 0), (10, 0, 0),
(10, 0, 3), (3, 0, 3), (3, 0, 10) and (0, 0, 10); extrude it along +y from y = 0 to y = 10. The
block has one concave edge: the inside corner, the line x = 3, z = 3 along y, where the face
z = 3 (from x = 3 to x = 10) meets the face x = 3 (from z = 3 to z = 10).

Chamfer that inside edge with the same distance 7 on both of its faces, adding material: the
chamfer is the plane through the lines (10, y, 3) and (3, y, 10), the far edges of both faces, so
it consumes both faces entirely. The section becomes the pentagon through (0, 0), (10, 0),
(10, 3), (3, 10) and (0, 10), in xz.
