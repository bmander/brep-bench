+++
title = "A plate with a shallow round recess grown past the recess's radius"
tags = ["offset", "grow", "analytic", "torus", "vanishing-face", "adversarial"]
origin = "adversarial: offset/drilled-plate-grown-past-its-hole with the hole made blind, a recess 1 deep"

[volume]
value = 8554.14919531404
exact = "V₀ − 2π((3 − √7)²/2 + 15√7/2 − 24·asin(√7/4) − 5/3), V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 4, d = 4"
tex = 'V_0 - 2\pi\left(\tfrac{(3 - \sqrt{7})^2}{2} + \tfrac{15\sqrt{7}}{2} - 24\arcsin\tfrac{\sqrt{7}}{4} - \tfrac{5}{3}\right),\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad a = b = 20,\ c = 4,\ d = 4'

[area]
value = 2428.3825924296486
exact = "A₀ − 9π + π(3 − √7)² + 8π(3·asin(√7/4) − 1), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², a = b = 20, c = 4, d = 4"
tex = 'A_0 - 9\pi + \pi(3 - \sqrt{7})^2 + 8\pi\left(3\arcsin\tfrac{\sqrt{7}}{4} - 1\right),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2'
+++

The plate of `offset/drilled-plate-grown`, from (−10, −10, 0) to (10, 10, 4), with a round recess
of radius 3 sunk 1 into its top about the z axis (a flat floor at z = 3), grown by 4, more than
the recess's radius: the solid of every point within 4 of the plate.

The plate grows as a box does: each face moves 4 outward, each edge becomes a quarter of a
cylinder of radius 4, each corner an eighth of a ball. The recess is gone: every point of it is
within 3 of its wall. Over it, though, a point is within 4 of the plate if it is within 4 of the
recess's rim or of its floor, and only then, so the grown top, at z = 8, dips over the recess: a
flat floor at z = 7, 4 above the recess's floor, out to radius 3 − √7 ≈ 0.354, and from there the
inner side of a tube of radius 4 about the rim's circle (of radius 3, at z = 4), which rises to
meet the flat of the grown top along the circle of radius 3.

**As a set:**

$$\Bigl(\bigl([-10,10]^2\times[0,4]\bigr) \setminus \{\,\rho \le 3,\ z \ge 3\,\}\Bigr) \oplus B_{4}$$
