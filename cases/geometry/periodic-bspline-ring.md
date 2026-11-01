+++
title = "A ring turned from a periodic B-spline"
tags = ["geometry", "nurbs", "bspline", "periodic", "revolution"]

[volume]
value = 1884.9555921538758
exact = "Pappus: the section's area (10/3)·3² = 30 (a square of half-side 3 less ⅓ of each corner's triangle), times the path of its centroid, 2π·10 (the section is symmetric about its centre): 600π"
tex = '2\pi\cdot 10\cdot\tfrac{10}{3}\cdot 3^2 = 600\pi'

[area]
value = 1223.8829974910582
exact = "Pappus: the section's perimeter, four parabolic arcs each 3(1 + ln(1 + √2)/√2) long, times 2π·10: 240π(1 + ln(1 + √2)/√2)"
tex = '2\pi\cdot 10\cdot 12\left(1 + \frac{\ln\left(1 + \sqrt 2\right)}{\sqrt 2}\right)'
+++

A ring about the z axis, its section a closed, periodic B-spline curve turned a whole turn about
the axis.

The section lies in the xz plane. It is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of corners (13, 0, 3), (7, 0, 3),
(7, 0, −3) and (13, 0, −3), taken in that order and closing back on the first: a square of side 6
centred at (10, 0, 0), rounded all the way to the middle of each side. The curve is four parabolic
arcs, each from the middle of one side to the middle of the next ((13, 0, 0) to (10, 0, 3), and so
on), tangent to the sides there. It is smooth everywhere and has no end.

The ring's surface is that curve turned about the z axis: periodic both ways, round the axis and
round the section, so a kernel has a closed surface with no boundary at all and must place both of
its seams itself.
