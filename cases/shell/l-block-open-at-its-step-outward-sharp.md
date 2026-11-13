+++
title = "An L-shaped block shelled outward open at its step, the walls beyond the opening kept"
tags = ["shell", "outward", "planar", "concave", "sharp-edge"]

[volume]
value = 1868
exact = "22·244 − 3500; 244 is the area of the moved faces' outline"
tex = '22\cdot 244 - 3500'

[area]
value = 3814
exact = "(1650 + 20) inside + (2·244 + 22·73) outside + 50 rim"
tex = '1670 + 2094 + 50'
+++

The L-shaped block of `shell/l-block-open-at-its-step-sharp`: its outline in the xy plane the
polygon through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z.
Shell it outward to a wall 1 thick, open at the face y = 5, with its edges kept sharp. The block's
faces other than the opening become the inside of the wall; the outside is each of them moved 1
outward along its normal, the moved faces extended until they meet, and cut off by the plane of
the opening, y = 5, which is the rim, only where the opening is: the tall leg beyond that plane
keeps its walls.

The moved faces' outline is the polygon through (−1, −1), (21, −1), (21, 21), (14, 21), (14, 5)
and (−1, 5), from z = −1 to z = 21. The wall of the face x = 15, moved out to x = 14, stands on the
opening's edge: its foot, the strip 14 ≤ x ≤ 15 of the plane y = 5, is a face of the result. A
kernel that cuts the whole half-space beyond the opening's plane away takes the tall leg's walls
with it and is wrong.

The same part built in Onshape, Shell, 1 mm, opposite direction, the face y = 5 removed, measures
1868 and 3814.

**As a set:**

$$\Bigl(\operatorname{poly}\bigl((-1,-1),(21,-1),(21,21),(14,21),(14,5),(-1,5)\bigr)\times[-1,21]\Bigr)
\setminus K,\quad K = \operatorname{poly}\bigl((0,0),(20,0),(20,20),(15,20),(15,5),(0,5)\bigr)\times[0,20]$$
