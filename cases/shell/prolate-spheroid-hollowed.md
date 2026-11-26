+++
title = "A prolate spheroid hollowed to a wall 1 thick"
tags = ["shell", "inward", "closed", "ellipsoid", "spheroid", "revolution", "adversarial"]
origin = "adversarial: shell/sphere-hollowed with a prolate spheroid in place of the ball"

[volume]
value = 145.4634119356697
exact = "S d − M d² + 4πd³/3: the spheroid less its inner parallel body (Steiner, d = 1 below the least radius of curvature a²/c = 3/2), S = 2πa²(1 + (c/(ae)) asin e) = 18π + 8√3π², M = 2πc + (2πa²/√(c² − a²)) artanh(√(c² − a²)/c) = 12π + 2√3π ln(2 + √3), a = 3, c = 6, e = √3/2"
tex = 'Sd - Md^2 + \tfrac{4}{3}\pi d^3,\quad S = 18\pi + 8\sqrt{3}\,\pi^2,\ M = 12\pi + 2\sqrt{3}\,\pi\ln(2 + \sqrt{3}),\ d = 1'

[area]
value = 295.1156140761258
exact = "S + (S − 2Md + 4πd²): the outside and the cavity's wall"
tex = 'S + \left(S - 2Md + 4\pi d^2\right)'
+++

The solid prolate spheroid (x² + y²)/9 + z²/36 ≤ 1, centered on the origin about the z axis: the
ellipse x²/9 + z²/36 ≤ 1 of the xz plane turned a whole turn about the z axis, its long axis the
axis of the turn, of radius 3 at its equator and reaching to z = ±6 at its poles. Hollow it to a
closed wall 1 thick, with no opening: the spheroid less every point of it more than 1 from its
outside.

The outside is the spheroid's surface, unmoved. The cavity is bounded by that surface moved 1
inward along its normals: its equator at radius 2 and its poles at z = ±5. The spheroid's surface
curves most tightly at its poles, where its radius of curvature is a²/c = 3/2, more than the
wall's thickness, so the moved surface is smooth and does not cross itself; it is not an
ellipsoid. The result is one solid with two faces, the cavity's wall facing inward.

The cavity is the spheroid's inner parallel body at d = 1, whose volume and area are Steiner's
polynomials at −d: V − Sd + Md² − 4πd³/3 and S − 2Md + 4πd², where V = 72π is the spheroid's
volume, S its area and M the integral of its surface's mean curvature, (κ₁ + κ₂)/2: the
parallels' curvature integrates to 4πc and the meridians' to
4πa²·artanh(√(c² − a²)/c)/√(c² − a²), half their sum being M.

**As a set:**

$$K \setminus (K \ominus B_1),\quad K = \{\,\tfrac{\rho^2}{9} + \tfrac{z^2}{36} \le 1\,\}$$
