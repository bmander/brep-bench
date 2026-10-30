+++
title = "A block whose top is chamfered into a ridge"
tags = ["chamfer", "vanishing-face"]

[volume]
value = 200.0
exact = "10·(4·4 + 4·2/2): the section is a 4 × 4 square under a right isosceles triangle"
tex = '10\left(4\cdot 4 + \tfrac{4\cdot 2}{2}\right)'

[area]
value = 216.5685424949238
exact = "2·20 + 40 + 2·40 + 2·2√2·10 = 160 + 40√2"
tex = '2\cdot 20 + 40 + 2\cdot 40 + 2\cdot 2\sqrt{2}\cdot 10 = 160 + 40\sqrt{2}'
+++

A block from the origin to (10, 4, 6). Chamfer the two edges of its top face z = 6 that run
along x, the one at y = 0 and the one at y = 4, each with the same distance 2 on both of its
faces: half the top face's width of 4.

The chamfer of the edge at y = 0 is the plane through the lines y = 2, z = 6 and y = 0, z = 4,
and the other's the plane through y = 2, z = 6 and y = 4, z = 4, so the top face vanishes
entirely: the two chamfers meet at right angles along the ridge y = 2, z = 6, and the block's
ends x = 0 and x = 10 are pentagons, a 4 × 4 square under a triangle.
