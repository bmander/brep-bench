+++
title = "A flanged drum shrunk until its flange is gone"
tags = ["offset", "shrink", "analytic", "torus", "self-intersection", "face-removed"]

[volume]
value = 624.3662286501782
exact = "π·7²·4 + 2π(16a − 8(a√(1 − a²) + asin a) − a³/3), a = 0.8"
tex = '\pi\cdot 7^2\cdot 4 + 2\pi\left(16a - 8\left(a\sqrt{1 - a^2} + \arcsin a\right) - \tfrac{1}{3}a^3\right),\quad a = 0.8'

[area]
value = 496.6023797475794
exact = "2π(49 + 7(4 − 2a) + 2(8 asin a − a)), a = 0.8"
tex = '2\pi\left(49 + 7(4 - 2a) + 2(8\arcsin a - a)\right),\quad a = 0.8'
+++

A drum: the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a thin flange round its
side, a ring from radius 8 out to radius 11 and from z = 2.2 to z = 3.8, 1.6 thick. Shrink it by
1: the solid of every point at least 1 from the flanged drum's outside.

Every point of the flange is within 0.8 of its top or bottom face, so shrinking takes the whole
flange away: its faces, moved 1 inward from each side, cross each other and are gone. The drum's
top and bottom move in to z = 5 and z = 1 and its side to radius 7, with its outer edges sharp.
Where the flange met the drum, its two inside edges (the circles of radius 8 at z = 2.2 and
z = 3.8) become concave pieces of tori, tubes of radius 1 about those circles, running from the
side at radius 7 outward and meeting each other in a sharp ridge, the circle of radius 7.4 at
z = 3. So a low ridge, 0.4 high, is all that is left of the flange.
