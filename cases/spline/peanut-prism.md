+++
title = "A peanut with a thin waist, a periodic spline extruded"
tags = ["spline", "extrude", "periodic", "sliver"]

[volume]
value = 2472.152777777778
exact = "10 × the section's area, 10·35599/144 = 177995/72: ½∮(x dy − y dx) over the twelve spans, each a polynomial (Green's theorem)"

[area]
value = 1319.2165440112476
exact = "2·35599/144 for the ends + 10·L for the side, L the curve's length, which has no closed form: OCCT's length of the curve (82.47859884564615) and cad-kernel's (82.47859884549231, its side face over 10) agree to 2e-12; the mean. OCCT's own total is off (its integration of the side face is), cad-kernel's whole solid agrees with this to 1e-12"
+++

A prism 10 tall standing on the xy plane, from z = 0 to z = 10, whose section is a peanut: the
region inside a closed, uniform, periodic cubic B-spline curve in the plane z = 0.

The curve's twelve control points, in order (counter-clockwise round the peanut), are
(14, 0), (12, 8), (4, 6), (0, −21/8), (−4, 6), (−12, 8), (−14, 0), (−12, −8), (−4, −6),
(0, 21/8), (4, −6) and (12, −8), and after the twelfth the curve comes back to the first. Its
knots are uniform and it is periodic, so the curve is C² everywhere, with no start or end: each
of its twelve spans is the uniform cubic B-spline segment of four consecutive control points
(wrapping round), the span from Pₖ, Pₖ₊₁, Pₖ₊₂, Pₖ₊₃ being
((1 − t)³Pₖ + (3t³ − 6t² + 4)Pₖ₊₁ + (−3t³ + 3t² + 3t + 1)Pₖ₊₂ + t³Pₖ₊₃) / 6 for t from 0 to 1.

The two waist control points lie on the far side of the x axis from the boundary they pull on
(the control polygon crosses itself there), so the curve itself does not cross: the peanut's two
lobes, about 28 across in all and 12 tall, are joined by a waist only 0.5 wide, the curve
passing through (0, 0.25) and (0, −0.25). The section is one region, and the prism's side is a
single smooth face round it.
