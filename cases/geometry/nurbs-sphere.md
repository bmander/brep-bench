+++
title = "A ball bounded by one NURBS surface with two poles"
tags = ["geometry", "nurbs", "rational", "pole", "degenerate-edge"]

[volume]
value = 523.5987755982989
exact = "4πr³/3, r = 5"
tex = '\tfrac{4}{3}\pi r^3,\quad r = 5'

[area]
value = 314.1592653589793
exact = "4πr²"
tex = '4\pi r^2'
+++

A ball of radius 5 centred on the origin, its whole surface one rational B-spline surface (a
NURBS), the standard biquadratic sphere: a 9 × 5 net whose first and last rows each collapse to a
single point, a pole.

The surface is the tensor product of two rational quadratic curves. Round the axis (u), the unit
circle of nine control points cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1),
(1, −1), (1, 0), with weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots
(0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Along the meridian (v), the semicircle from the south pole to
the north of five control points (ρⱼ, zⱼ) = (0, −5), (5, −5), (5, 0), (5, 5), (0, 5), with weights
vⱼ = 1, √2/2, 1, √2/2, 1, on the knots (0, 0, 0, ½, ½, 1, 1, 1). The net's control point (i, j) is
(ρⱼ·cᵢ, zⱼ) = (ρⱼ cᵢₓ, ρⱼ cᵢᵧ, zⱼ), with weight wᵢvⱼ.

So every control point with j = 0 is (0, 0, −5) and every one with j = 4 is (0, 0, 5): the
surface's whole edge v = 0 is the south pole, and its edge v = 1 the north. Its edges u = 0 and
u = 1 are the same meridian, in the half plane y = 0, x ≥ 0, which is the surface's seam. Build the
ball from that one face, closed on its seam and at its poles; do not use a sphere.
