+++
title = "A flat elliptic cylinder grown by 1"
tags = ["offset", "grow", "ellipse", "canal", "adversarial"]
origin = "adversarial: offset/elliptic-cylinder-grown with the ellipse 10 × 2 in place of 5 × 3"

[volume]
value = 1275.792845973496
exact = "V + Sd + Md² + 4πd³/3 (Steiner), V = πabh, S = 2πab + ph, M = πh + πp/2, p = 4aE(1 − b²/a²), a = 10, b = 2, h = 10, d = 1"
tex = 'V + Sd + Md^2 + \tfrac{4}{3}\pi d^3,\quad V = \pi abh,\ S = 2\pi ab + ph,\ M = \pi h + \tfrac{\pi p}{2},\ p = 4aE\left(1 - \tfrac{b^2}{a^2}\right),\ a = 10,\ b = 2,\ h = 10,\ d = 1'

[area]
value = 753.2728237784894
exact = "S + 2Md + 4πd²"
tex = 'S + 2Md + 4\pi d^2'
+++

The elliptic cylinder x²/100 + y²/4 ≤ 1, the ellipse in the plane z = 0 with semi-axes 10 along x
and 2 along y, extruded straight up to z = 10, grown by 1: the solid of every point within 1 of
it. It is `offset/elliptic-cylinder-grown` with a flatter ellipse.

The ends move to z = −1 and z = 11. The side moves out along its normals to the curve at distance
1 outside the ellipse, extruded from z = 0 to z = 10. The ellipse curves most tightly at the ends
of its long axis, where its radius of curvature is b²/a = 0.4, less than the distance grown; but
the curve moved outward does not fold, so the side is smooth. Each rim becomes the outer quarter of
a tube of radius 1 about the rim's ellipse.

E is the complete elliptic integral of the second kind, of parameter m = 1 − b²/a² = 24/25, so p is
the ellipse's perimeter, about 42.0201; M is the side's mean curvature integrated over it, πh, plus
each rim's length times half its turning angle, π/4.

**As a set:**

$$\{\,\tfrac{x^2}{100} + \tfrac{y^2}{4} \le 1,\ 0 \le z \le 10\,\} \oplus B_{1}$$
