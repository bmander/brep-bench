+++
title = "A ball's cap turned from a weighted quadratic arc"
tags = ["spline", "revolve", "rational", "weights", "pole"]

[volume]
value = 3534.2917352885174
exact = "πh²(3R − h)/3 = 1125π, R = 10, h = 15: the weighted arc is exactly a circular arc of radius 10 about the origin, so the solid is a cap of a ball"
tex = '\tfrac{\pi h^2}{3}(3R - h) = 1125\pi,\quad R = 10,\ h = 15'

[area]
value = 1178.0972450961724
exact = "2πRh + π(5√3)² = 300π + 75π = 375π: the cap's spherical zone and the flat disc it stands on"
tex = '2\pi R h + \pi\left(5\sqrt{3}\right)^2 = 300\pi + 75\pi = 375\pi'
+++

A solid made by turning a plane region a full turn about the z axis. The region lies in the xz
plane, on the side x ≥ 0, and is bounded by three pieces: the segment of the z axis from
(0, 0, −5) to (0, 0, 10); the segment from (0, 0, −5) to (5√3, 0, −5); and a rational quadratic
Bézier curve, one segment, from (5√3, 0, −5) to (0, 0, 10).

The curve's three control points are P0 = (5√3, 0, −5), P1 = (10√3, 0, 10) and
P2 = (0, 0, 10), with weights 1, ½ and 1: the point at parameter t is
(B0·P0 + ½B1·P1 + B2·P2) / (B0 + ½B1 + B2), where B0 = (1 − t)², B1 = 2t(1 − t) and B2 = t². The
weights matter: with them the curve is exactly the arc of the circle of radius 10 about the
origin, turning 120° from (5√3, 0, −5) over its widest point (10, 0, 0) to the top of the circle;
without them it would be a parabola.

So the solid stands on a flat disc of radius 5√3 at z = −5, bulges out to radius 10 at z = 0
and closes over the top at (0, 0, 10), where the curve meets the axis square to it.
