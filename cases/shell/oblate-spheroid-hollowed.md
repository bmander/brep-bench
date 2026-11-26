+++
title = "An oblate spheroid hollowed to a wall 1 thick"
tags = ["shell", "inward", "closed", "ellipsoid", "spheroid", "revolution", "adversarial"]
origin = "adversarial: shell/sphere-hollowed with an oblate spheroid in place of the ball"

[volume]
value = 251.94126154155704
exact = "S d − M d² + 4πd³/3: the spheroid less its inner parallel body (Steiner, d = 1 below the least radius of curvature c²/a = 3/2), S = 72π + 12√3π ln(2 + √3), M = 6π + 8√3π²/3, a = 6, c = 3"
tex = 'Sd - Md^2 + \tfrac{4}{3}\pi d^3,\quad S = 72\pi + 12\sqrt{3}\,\pi\ln(2 + \sqrt{3}),\ M = 6\pi + \tfrac{8\sqrt{3}}{3}\pi^2,\ d = 1'

[area]
value = 508.07131328790047
exact = "S + (S − 2Md + 4πd²): the outside and the cavity's wall"
tex = 'S + \left(S - 2Md + 4\pi d^2\right)'
+++

The solid oblate spheroid of `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, centered on the
origin about the z axis, hollowed to a closed wall 1 thick, with no opening: the spheroid less
every point of it more than 1 from its outside.

The outside is the spheroid's surface, unmoved. The cavity is bounded by that surface moved 1
inward along its normals: its equator at radius 5 and its poles at z = ±2. The spheroid's surface
curves most tightly at its equator, where its meridian's radius of curvature is c²/a = 3/2, more
than the wall's thickness, so the moved surface is smooth and does not cross itself; it is not an
ellipsoid. The result is one solid with two faces, the cavity's wall facing inward.

The cavity is the spheroid's inner parallel body at d = 1, whose volume and area are Steiner's
polynomials at −d: V − Sd + Md² − 4πd³/3 and S − 2Md + 4πd², where V = 144π, S and M are the
spheroid's volume, area and integral of mean curvature (as in `offset/oblate-spheroid-grown`).

**As a set:**

$$K \setminus (K \ominus B_1),\quad K = \{\,\tfrac{\rho^2}{36} + \tfrac{z^2}{9} \le 1\,\}$$
