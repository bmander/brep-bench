+++
title = "A hemisphere whose rim is chamfered with two distances"
tags = ["chamfer", "asymmetric", "sphere", "cone"]

[volume]
value = 234.39614467902140
exact = "250π/3 − π(25h − h³/3) + πh(3² + 3·4.9 + 4.9²)/3, h = 3√11/10: π(25000 − 789√11)/300"
tex = '\tfrac{250\pi}{3} - \pi\left(25h - \tfrac{h^3}{3}\right) + \tfrac{\pi h}{3}\left(3^2 + 3\cdot 4.9 + 4.9^2\right),\quad h = \tfrac{3\sqrt{11}}{10}'

[area]
value = 207.32544246695442
exact = "π3² + 2π·5(5 − h) + π(3 + 4.9)√(1.9² + h²), h = 3√11/10: π(2950 − 150√11 + 79√115)/50"
tex = '9\pi + 10\pi(5 - h) + 7.9\pi\sqrt{1.9^2 + h^2},\quad h = \tfrac{3\sqrt{11}}{10}'
+++

The upper half of a ball of radius 5 centered on the origin, z ≥ 0. Chamfer the circle where its
flat face z = 0 meets the ball, with two distances: 2 on the flat face and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge): the chamfer meets the flat face on the circle of radius 3, 2 in from
the rim, and the ball on the circle whose points are 1 in a straight line from the rim, the chord
1 long. That chord turns 2·asin(1/10) about the ball's center, so the circle on the ball has radius
4.9 and height h = 3√11/10 ≈ 0.995. The chamfer is the cone between the two circles, and the solid
is the hemisphere cut by it: the flat face shrinks to the disk of radius 3, and the ball keeps
everything above z = h.

(Measured along the ball as an arc, 1 would stop about 1/600 short of that circle: the straight
line is what is meant.)

**As a set:**

$$\{\,\lVert p\rVert \le 5,\ z \ge 0,\ 19z \ge 3\sqrt{11}\,(\rho - 3)\,\}$$
