+++
title = "A cone bounded by a NURBS surface with a pole"
tags = ["geometry", "nurbs", "rational", "pole", "degenerate-edge", "apex"]

[volume]
value = 261.79938779914943
exact = "πr²h/3 = 250π/3, r = 5, h = 10"
tex = '\tfrac13\pi r^2 h = \tfrac{250\pi}{3},\quad r = 5,\ h = 10'

[area]
value = 254.160184615763
exact = "the base πr² = 25π + the side πr√(r² + h²) = 25√5π"
tex = '\pi r^2 + \pi r\sqrt{r^2 + h^2} = 25\pi + 25\sqrt 5\,\pi'
+++

A solid cone standing on the disc of radius 5 about the z axis in the plane z = 0, its apex at
(0, 0, 10), whose side is one rational B-spline surface (a NURBS) with a pole at the apex, closed by
a flat disc at its base.

The side is rational quadratic round the axis (u) and linear up it (v): a 9 × 2 net. Its first row
(v = 0) is the base circle of radius 5 as a rational quadratic: the nine control points 5cᵢ in the
plane z = 0, cᵢ = (1, 0), (1, 1), (0, 1), (−1, 1), (−1, 0), (−1, −1), (0, −1), (1, −1), (1, 0), with
weights wᵢ = 1, √2/2, 1, √2/2, 1, √2/2, 1, √2/2, 1, on the knots
(0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1). Its second row (v = 1) is nine copies of the apex (0, 0, 10),
with the same weights wᵢ, and v runs on the knots (0, 0, 1, 1). Each iso-u line is the straight
line from the circle to the apex, and the whole edge v = 1 of the surface collapses to the apex.

Build the cone from that face and the base's disc; do not use a cone.
