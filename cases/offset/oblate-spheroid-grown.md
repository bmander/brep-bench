+++
title = "An oblate spheroid grown by 1"
tags = ["offset", "grow", "ellipsoid", "spheroid", "revolution", "adversarial"]
origin = "adversarial: geometry/oblate-spheroid grown, a smooth convex body that is no sphere"

[volume]
value = 833.2012156257897
exact = "V + Sd + Md² + 4πd³/3 (Steiner), V = 144π, S = 72π + 12√3π ln(2 + √3), M = ∫H dA = 2πc + (2πa²/√(a² − c²)) atan(√(a² − c²)/c) = 6π + 8√3π²/3, a = 6, c = 3, d = 1"
tex = 'V + Sd + Md^2 + \tfrac{4}{3}\pi d^3,\quad V = 144\pi,\ S = 72\pi + 12\sqrt{3}\,\pi\ln(2 + \sqrt{3}),\ M = 2\pi c + \frac{2\pi a^2}{\sqrt{a^2 - c^2}}\arctan\frac{\sqrt{a^2 - c^2}}{c} = 6\pi + \tfrac{8\sqrt{3}}{3}\pi^2,\ a = 6,\ c = 3,\ d = 1'

[area]
value = 453.62475990208345
exact = "S + 2Md + 4πd²"
tex = 'S + 2Md + 4\pi d^2'
+++

The solid oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, centered on the
origin about the z axis, grown by 1: the solid of every point within 1 of it.

The spheroid is convex and smooth, so the grown solid is bounded by one smooth face, the spheroid's
surface moved 1 outward along its normals: its equator moves out to radius 7 and its poles to
z = ±4. That face is not an ellipsoid. Its section through the z axis is the parallel curve of the
ellipse x²/36 + z²/9 = 1 at distance 1.

For a convex body the volume of the grown solid is Steiner's polynomial in the distance d, whose
coefficients are the body's volume V, its surface area S and the integral M of its surface's mean
curvature, (κ₁ + κ₂)/2, over the surface; the grown solid's area is its derivative in d. For the
spheroid with equatorial radius a = 6 and polar radius c = 3, the parallels' curvature integrates
to 4πc and the meridians' to 4πa²·atan(√(a² − c²)/c)/√(a² − c²), half their sum being M.

**As a set:**

$$\{\,\tfrac{\rho^2}{36} + \tfrac{z^2}{9} \le 1\,\} \oplus B_{1}$$
