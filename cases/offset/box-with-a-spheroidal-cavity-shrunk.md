+++
title = "A box with a spheroidal cavity shrunk by 1"
tags = ["offset", "shrink", "void", "ellipsoid", "spheroid", "adversarial"]
origin = "adversarial: offset/box-shrunk with a closed oblate-spheroidal cavity inside, which grows"

[volume]
value = 4998.79878437421
exact = "(a − 2d)³ − (V + Sd + Md² + 4πd³/3): the cube shrunk less the cavity grown (Steiner), a = 20, d = 1, V = 144π, S = 72π + 12√3π ln(2 + √3), M = 6π + 8√3π²/3"
tex = '(a - 2d)^3 - \Bigl(V + Sd + Md^2 + \tfrac{4}{3}\pi d^3\Bigr),\quad a = 20,\ d = 1,\ V = 144\pi,\ S = 72\pi + 12\sqrt{3}\,\pi\ln(2 + \sqrt{3}),\ M = 6\pi + \tfrac{8\sqrt{3}}{3}\pi^2'

[area]
value = 2397.6247599020835
exact = "6(a − 2d)² + (S + 2Md + 4πd²)"
tex = '6(a - 2d)^2 + \Bigl(S + 2Md + 4\pi d^2\Bigr)'
+++

The cube from (−10, −10, −10) to (10, 10, 10) with a closed cavity at its middle, the oblate
spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, taken out of it: a solid bounded by
the cube's six faces outside and the spheroid's surface inside, facing into the cavity. Shrink it
by 1: keep every point of it at least 1 from its boundary.

Outside, the cube shrinks to the cube from (−9, −9, −9) to (9, 9, 9), its edges sharp. Inside, the
cavity grows to the spheroid's parallel body at distance 1: every point within 1 of the spheroid,
bounded by the spheroid's surface moved 1 outward along its normals, its equator at radius 7 and
its poles at z = ±4. That surface is smooth and is not an ellipsoid. It stays 2 from the shrunk
cube's faces, so the result is one solid with one closed cavity.

The volume and area of the grown cavity are Steiner's polynomials of the spheroid at d, as in
`offset/oblate-spheroid-grown`, with V = 144π, S and M the spheroid's volume, area and integral of
mean curvature.

**As a set:**

$$\Bigl([-10,10]^3 \setminus E\Bigr) \ominus B_{1},\quad E = \{\,\tfrac{\rho^2}{36} + \tfrac{z^2}{9} < 1\,\}$$
