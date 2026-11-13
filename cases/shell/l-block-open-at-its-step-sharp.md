+++
title = "An L-shaped block shelled inward open at its step, the wall rising past the opening kept"
tags = ["shell", "inward", "planar", "concave", "sharp-edge"]

[volume]
value = 1448
exact = "3500 − 18·114; 114 is the area of the moved walls' outline"
tex = '3500 - 18\cdot 114'

[area]
value = 2970
exact = "1650 outside + 48 rim + (72 + 324 + 324 + 54 + 252 + 2·114 + 18) inside"
tex = '1650 + 48 + 1272'
+++

An L-shaped block: its outline in the xy plane is the polygon through (0, 0), (20, 0), (20, 20),
(15, 20), (15, 5) and (0, 5), extruded 20 along +z. Its one concave edge is the vertical line
x = 15, y = 5, between the face y = 5 of the short leg and the face x = 15 of the tall one. Shell it
inward to a wall 1 thick, open at the face y = 5, with the walls' corners kept sharp. The outside of
the wall is the block's other faces, unmoved, and the inside is each of them moved 1 inward along
its normal, onto the plane offset 1 from it. The moved faces are extended or trimmed where they
meet one another, and run on until they meet the plane of the opening, which is left as a rim
between the outside and the inside.

The face x = 15 rises from the opening's edge, beyond its plane: it is not cut by the opening, and
its wall, moved 1 inward to x = 16, runs down to the opening's plane y = 5, where the wall's foot,
the strip 15 ≤ x ≤ 16, is a face of the cavity. The cavity is the polygon through (1, 1), (19, 1),
(19, 19), (16, 19), (16, 5) and (1, 5), from z = 1 to z = 19. A kernel that carries the opening on
past its plane (cutting the cavity from a slab beyond it) cuts into the wall at x = 15 and is
wrong.

The same part built in Onshape, Shell, 1 mm, the face y = 5 removed, measures 1448 and 2970.

**As a set:**

$$\bigl(\operatorname{poly}\bigl((0,0),(20,0),(20,20),(15,20),(15,5),(0,5)\bigr)\times[0,20]\bigr)
\setminus
\Bigl(\operatorname{poly}\bigl((1,1),(19,1),(19,19),(16,19),(16,5),(1,5)\bigr)\times[1,19]\Bigr)$$
