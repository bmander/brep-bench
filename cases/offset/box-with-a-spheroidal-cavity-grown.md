+++
title = "A box with a spheroidal cavity grown by 1"
tags = ["offset", "grow", "void", "ellipsoid", "spheroid", "adversarial"]
origin = "adversarial: offset/box-grown with a closed oblate-spheroidal cavity inside, which shrinks"

[volume]
value = 10392.236268844801
exact = "(a³ + 6a²d + 3πad² + 4πd³/3) − (V − Sd + Md² − 4πd³/3): the cube grown less the cavity shrunk (Steiner), a = 20, d = 1, V = 144π, S = 72π + 12√3π ln(2 + √3), M = 6π + 8√3π²/3"
tex = '\Bigl(a^3 + 6a^2d + 3\pi a d^2 + \tfrac{4}{3}\pi d^3\Bigr) - \Bigl(V - Sd + Md^2 - \tfrac{4}{3}\pi d^3\Bigr),\quad a = 20,\ d = 1,\ V = 144\pi,\ S = 72\pi + 12\sqrt{3}\,\pi\ln(2 + \sqrt{3}),\ M = 6\pi + \tfrac{8\sqrt{3}}{3}\pi^2'

[area]
value = 2985.441025012613
exact = "(6a² + 6πad + 4πd²) + (S − 2Md + 4πd²)"
tex = '\Bigl(6a^2 + 6\pi a d + 4\pi d^2\Bigr) + \Bigl(S - 2Md + 4\pi d^2\Bigr)'
+++

The cube from (−10, −10, −10) to (10, 10, 10) with a closed cavity at its middle, the oblate
spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, taken out of it: a solid bounded by
the cube's six faces outside and the spheroid's surface inside, facing into the cavity. Grow it by
1: the solid of every point within 1 of it.

Outside, the cube grows as a box does: each face moves 1 outward, each edge becomes a quarter of a
cylinder of radius 1, each corner an eighth of a ball. Inside, every point of the cavity within 1
of its wall is filled, so the cavity shrinks to the spheroid's inner parallel body at distance 1:
the points of the spheroid more than 1 from its surface, bounded by the spheroid's surface moved 1
inward along its normals, its equator at radius 5 and its poles at z = ±2. The spheroid's surface
curves most tightly at its equator, where its radius of curvature is 3/2, more than 1, so that
moved surface is smooth; it is not an ellipsoid. The grown solid is still one solid with one closed
cavity, which no face of the cube's comes near.

The volume and area of the cavity left are Steiner's polynomials of the spheroid at −d, as in
`shell/oblate-spheroid-hollowed`, with V = 144π, S and M the spheroid's volume, area and integral
of mean curvature.

**As a set:**

$$\Bigl([-10,10]^3 \setminus E\Bigr) \oplus B_{1},\quad E = \{\,\tfrac{\rho^2}{36} + \tfrac{z^2}{9} < 1\,\}$$
