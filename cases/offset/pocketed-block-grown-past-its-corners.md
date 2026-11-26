+++
title = "A pocketed block grown past its pocket's corner radius"
tags = ["offset", "grow", "analytic", "concave-edge", "vanishing-face", "adversarial"]
origin = "adversarial: offset/drilled-plate-grown-past-its-hole's dimple, met where a concave round (not a hole) closes"

[volume]
value = 25008.108658735356
exact = "V₀ − [(w − 2d)(h − 2d)(D − d) + whd − π(w + h)d²/2 + 8d³/3 − (4 − π)J], V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, J = r²(d − q) − 2r(πd²/4 − qr/2 − (d²/2)·asin(q/d)) + d²(d − q) − (d³ − q³)/3, q = √(d² − r²), a = 40, b = 30, c = 10, w = 20, h = 14, r = 2, D = 6, d = 3"
tex = 'V_0 - \Bigl[(w - 2d)(h - 2d)(D - d) + whd - \tfrac{\pi}{2}(w + h)d^2 + \tfrac{8}{3}d^3 - (4 - \pi)J\Bigr],\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad J = r^2(d - q) - 2r\Bigl(\tfrac{\pi d^2}{4} - \tfrac{qr}{2} - \tfrac{d^2}{2}\arcsin\tfrac{q}{d}\Bigr) + d^2(d - q) - \tfrac{d^3 - q^3}{3},\quad q = \sqrt{d^2 - r^2},\ a = 40,\ b = 30,\ c = 10,\ w = 20,\ h = 14,\ r = 2,\ D = 6,\ d = 3'

[area]
value = 5633.355635493959
exact = "A₀ − (wh − (4 − π)r²) + (w − 2d)(h − 2d) + (2(w + h) − 8d)(D − d) + π(w + h)d − 8d² + (2π − 8)d(r·asin(r/d) − (d − q)), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², with the values above"
tex = 'A_0 - \bigl(wh - (4 - \pi)r^2\bigr) + (w - 2d)(h - 2d) + \bigl(2(w + h) - 8d\bigr)(D - d) + \pi(w + h)d - 8d^2 + (2\pi - 8)\,d\Bigl(r\arcsin\tfrac{r}{d} - (d - q)\Bigr),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2'
+++

The block from (−20, −15, 0) to (20, 15, 10) with a pocket cut down into its top: the rectangle
from (−10, −7) to (10, 7), its four corners rounded to radius 2, from the top down to a flat
floor at z = 4. Grow it by 3, more than the pocket's corner radius: the solid of every point
within 3 of the pocketed block.

The block grows as a box does: each face moves 3 outward, each outer edge becomes a quarter of a
cylinder of radius 3, each corner an eighth of a ball. In the pocket, every point within 3 of
its walls or its floor is filled: the floor rises to z = 7, the walls come in 3, and the rounded
corners, of radius 2, close up, so the pocket left below the grown top is the rectangle from
(−7, −4) to (7, 4) with square corners, from z = 7 up to z = 10. Above the pocket, a point is
within 3 of the block only within 3 of the pocket's rim, so the grown top dips there: over the
straight parts of the rim the dip is bounded by quarter cylinders of radius 3 about the rim's
edges, and over each rounded corner by the inner side of a tube of radius 3 about the corner's
arc. Section by section, at a height u above the top (0 ≤ u ≤ 3) the opening left is the
pocket's outline shrunk by √(9 − u²): the rounded rectangle with its sides moved in that far and
its corners' radius 2 − √(9 − u²) where that is positive, square where it is not.

**As a set:**

$$\Bigl([-20,20]\times[-15,15]\times[0,10] \;\setminus\; \bigl(R \times [4, 10]\bigr)\Bigr) \oplus B_{3},\quad R = \bigl([-8,8]\times[-5,5]\bigr) \oplus B_2$$
