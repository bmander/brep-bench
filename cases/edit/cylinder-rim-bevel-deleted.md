+++
title = "A cylinder's beveled rim deleted: the top and the side meet again"
tags = ["edit", "delete-face", "chamfer", "cone", "cylinder", "analytic"]

[volume]
value = 1130.9733552923256
exact = "π·6²·10 = 360π"
tex = '\pi\cdot 6^2\cdot 10 = 360\pi'

[area]
value = 603.1857894892403
exact = "2π·6² + 2π·6·10 = 192π"
tex = '2\pi\cdot 6^2 + 2\pi\cdot 6\cdot 10 = 192\pi'
+++

A cylinder of radius 6 on the z axis from z = 0 to z = 10, with its top rim beveled 2 back on each
face: the cylinder from z = 0 to z = 8 with a cone frustum on it, from radius 6 at z = 8 to radius 4
at z = 10. The bevel is the frustum's side.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on from radius 4 and the side from z = 8 until they meet in the circle of radius 6 at z = 10.

The result is the cylinder of radius 6 from z = 0 to z = 10.

**As a set:**

$$\{\,x^2 + y^2 \le 6^2,\ 0 \le z \le 10\,\}$$
