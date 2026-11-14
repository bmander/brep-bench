+++
title = "A boss's rounded root deleted: the boss stands on the plate with a sharp foot"
tags = ["edit", "delete-face", "fillet", "torus", "cylinder", "concave-edge", "analytic"]

[volume]
value = 1030.4423903774522
exact = "π·8²·4 + π·3²·8 = 328π: the plate and the boss"
tex = '\pi\cdot 8^2\cdot 4 + \pi\cdot 3^2\cdot 8 = 328\pi'

[area]
value = 753.98223686155038
exact = "2π·8² + 2π·8·4 + 2π·3·8 = 240π: the plate's surface, the boss's foot taken from it and its top put back, and the boss's side"
tex = '2\pi\cdot 8^2 + 2\pi\cdot 8\cdot 4 + 2\pi\cdot 3\cdot 8 = 240\pi'
+++

A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss standing on it,
the cylinder of radius 3 from z = 0 to z = 8. The concave edge where the boss meets the plate, the
circle of radius 3 at z = 0, is rounded to radius 1: the round is a quarter of a torus whose tube
has radius 1 about the circle of radius 4 at z = 1, tangent to the plate's top along the circle of
radius 4 and to the boss's side along the circle at z = 1.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The
plate's top runs in from radius 4 and the boss's side down from z = 1 until they meet in the circle
of radius 3 at z = 0, and the material the round added is gone.

The result is the plate with the boss on it, the edge at the boss's foot sharp.

**As a set:**

$$\{\,x^2 + y^2 \le 8^2,\ -4 \le z \le 0\,\} \cup \{\,x^2 + y^2 \le 3^2,\ 0 \le z \le 8\,\}$$
