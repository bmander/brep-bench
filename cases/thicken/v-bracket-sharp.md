+++
title = "A sheet creased to 60°, thickened outside its crease, the corner kept sharp"
tags = ["thicken", "sheet", "planar", "crease", "adversarial"]
origin = "adversarial: thicken/angle-bracket-sharp with its crease closed from 90° to 60°"

[volume]
value = 173.85640646055103
exact = "w(2lt + t²·cot(α/2)) = 8(20 + √3), l = 10, t = 1, w = 8, α = 60°"
tex = 'w\left(2lt + t^2\cot\tfrac{\alpha}{2}\right) = 8\left(20 + \sqrt{3}\right),\quad l = 10,\ t = 1,\ w = 8,\ \alpha = 60^\circ'

[area]
value = 407.1769145362398
exact = "2(2lt + t²·cot(α/2)) + w(4l + 2t + 2t·cot(α/2)) = 376 + 18√3"
tex = '2\left(2lt + t^2\cot\tfrac{\alpha}{2}\right) + w\left(4l + 2t + 2t\cot\tfrac{\alpha}{2}\right) = 376 + 18\sqrt{3}'
+++

A sheet of two flat faces meeting at 60° along a crease: the floor, the rectangle in the xy plane
from (0, 0, 0) to (10, 8, 0), and the wall, the rectangle through (0, 0, 0), (0, 8, 0),
(5, 8, 5√3) and (5, 0, 5√3), which rises from the floor's edge along the y axis at 60° to the
floor, leaning over it. They share the edge from (0, 0, 0) to (0, 8, 0).

Thicken it by 1 to the outside of its crease (below the floor, and above the wall on the side away
from the floor), the corner kept sharp: each face moves 1 out along its normal, the two moved faces
extended until they meet, and the sheet's other edges walled along its normals.

The thickened sheet's section in any plane y = const is the hexagon through (0, 0), (10, 0),
(10, −1), (−√3, −1), (5 − √3/2, 5√3 + 1/2) and (5, 5√3), each given as (x, z), extruded from
y = 0 to y = 8: the floor's strip 1 thick, the wall's strip 1 thick, and between them the sharp
corner, the kite of area cot 30° = √3 whose far point (−√3, −1) is where the moved floor z = −1
meets the moved wall.

**As a set:**

$$\operatorname{poly}\bigl((0,0), (10,0), (10,-1), (-\sqrt{3},-1), (5 - \tfrac{\sqrt{3}}{2}, 5\sqrt{3} + \tfrac{1}{2}), (5, 5\sqrt{3})\bigr)_{xz} \times [0, 8]_y$$
