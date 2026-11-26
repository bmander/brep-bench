+++
title = "An L-shaped plate with its inside corner rounded, grown past the round's radius"
tags = ["offset", "grow", "analytic", "concave-edge", "vanishing-face", "adversarial"]
origin = "adversarial: offset/l-block-grown with its inside corner first rounded to 4, then grown by 5"

[volume]
value = 9324.099510940506
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3) + 2(1 − π/4)J, J = r²(d − q) − 2r(πd²/4 − qr/2 − (d²/2)·asin(q/d)) + d²(d − q) − (d³ − q³)/3, q = √(d² − r²), A = 150, P = 70, t = 8, k = 5π/4 − 1, r = 4, d = 5"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right) + 2\left(1 - \tfrac{\pi}{4}\right)J,\quad J = r^2(d - q) - 2r\Bigl(\tfrac{\pi d^2}{4} - \tfrac{qr}{2} - \tfrac{d^2}{2}\arcsin\tfrac{q}{d}\Bigr) + d^2(d - q) - \tfrac{d^3 - q^3}{3},\quad q = \sqrt{d^2 - r^2},\ A = 150,\ P = 70,\ t = 8,\ k = \tfrac{5\pi}{4} - 1,\ r = 4,\ d = 5'

[area]
value = 2485.947167501042
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²) + 2(1 − π/4)(r² − 2dr·asin(r/d) + 2d(d − q)), with the values above"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right) + 2\left(1 - \tfrac{\pi}{4}\right)\left(r^2 - 2dr\arcsin\tfrac{r}{d} + 2d(d - q)\right)'
+++

The L-shaped plate of `offset/l-block-grown`, whose outline in the xy plane is the polygon
through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8 along +z, with its
inside corner's edge, the concave edge along (5, 5), rounded to radius 4 (a quarter cylinder about
the line x = 9, y = 9, touching the faces y = 5 and x = 5). Grow it by 5, more than the round's
radius: the solid of every point within 5 of the plate.

Its section at a height from z = 0 to z = 8 is the outline (with its rounded corner) grown by 5:
each side moved out 5, the five convex corners rounded to radius 5, and the inside corner sharp,
the round, of radius 4, having closed up. Above the top (and below the bottom), at a height u,
the section is the outline grown by s = √(25 − u²): while s is at least 4 it is the sharp-cornered
L grown by s, and once s is less than 4 its inside corner is the round's arc grown by s, of
radius 4 − s about the line x = 9, y = 9, which leaves more than the sharp corner would. Where s is
under 4 the grown plate's surface over the inside corner is the outer side of a tube of radius 5
about the round's top (and bottom) arc.

The section at height u is $A + Ps + ks^2 + (1 - \pi/4)\,\max(0,\ 4 - s)^2$, where A and P are the
sharp-cornered outline's area and perimeter.

**As a set:**

$$\Bigl(\Bigl(L \cup \bigl([5,9]^2 \setminus B_4\bigl((9,9)\bigr)\bigr)\Bigr) \times [0, 8]\Bigr) \oplus B_5,\quad L = \operatorname{poly}\bigl((0,0), (20,0), (20,5), (5,5), (5,15), (0,15)\bigr)$$
