+++
title = "A plate with a round-cornered window grown past the corners' radius"
tags = ["offset", "grow", "analytic", "concave-edge", "vanishing-face", "adversarial"]
origin = "adversarial: offset/drilled-plate-grown-past-its-hole with the hole a window whose rounded corners, not the window, close"

[volume]
value = 14575.527268062979
exact = "V₀ − [c(w − 2d)(h − 2d) + 2(whd − π(w + h)d²/2 + 8d³/3 − (4 − π)J)], V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, J = r²(d − q) − 2r(πd²/4 − qr/2 − (d²/2)·asin(q/d)) + d²(d − q) − (d³ − q³)/3, q = √(d² − r²), a = 40, b = 30, c = 4, w = 20, h = 14, r = 2, d = 3"
tex = 'V_0 - \Bigl[c(w - 2d)(h - 2d) + 2\Bigl(whd - \tfrac{\pi}{2}(w + h)d^2 + \tfrac{8}{3}d^3 - (4 - \pi)J\Bigr)\Bigr],\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad J = r^2(d - q) - 2r\Bigl(\tfrac{\pi d^2}{4} - \tfrac{qr}{2} - \tfrac{d^2}{2}\arcsin\tfrac{q}{d}\Bigr) + d^2(d - q) - \tfrac{d^3 - q^3}{3},\quad q = \sqrt{d^2 - r^2},\ a = 40,\ b = 30,\ c = 4,\ w = 20,\ h = 14,\ r = 2,\ d = 3'

[area]
value = 4580.5521262063522
exact = "A₀ − 2(wh − (4 − π)r²) + c(2(w + h) − 8d) + 2(π(w + h)d − 8d² + (2π − 8)d(r·asin(r/d) − (d − q))), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², with the values above"
tex = 'A_0 - 2\bigl(wh - (4 - \pi)r^2\bigr) + c\bigl(2(w + h) - 8d\bigr) + 2\Bigl(\pi(w + h)d - 8d^2 + (2\pi - 8)\,d\Bigl(r\arcsin\tfrac{r}{d} - (d - q)\Bigr)\Bigr),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2'
+++

The plate from (−20, −15, 0) to (20, 15, 4) with a window cut right through it along z: the
rectangle from (−10, −7) to (10, 7) with its four corners rounded to radius 2. Grow it by 3,
more than the corners' radius but less than half the window's width: the solid of every point
within 3 of the plate.

The plate grows as a box does: each face moves 3 outward, each outer edge becomes a quarter of a
cylinder of radius 3, each corner an eighth of a ball. The window stays open but narrows: within
the plate's thickness its walls come in 3, to the rectangle from (−7, −4) to (7, 4), whose
corners are square, the rounded corners (of radius 2) having closed up. Over and under the
window, at a height u beyond the plate's face (0 ≤ u ≤ 3), the opening is the window's outline
shrunk by √(9 − u²): its sides moved in that far and its corners' radius 2 − √(9 − u²) where that
is positive, square where it is not. So the grown faces round the window's straight rim in
quarter cylinders of radius 3 about its edges, and its rounded corners in the outer side of a tube
of radius 3 about each corner's arc.

**As a set:**

$$\Bigl([-20,20]\times[-15,15]\times[0,4] \;\setminus\; \bigl(R \times [0, 4]\bigr)\Bigr) \oplus B_{3},\quad R = \bigl([-8,8]\times[-5,5]\bigr) \oplus B_2$$
