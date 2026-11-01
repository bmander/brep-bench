+++
title = "A prism on a periodic B-spline"
tags = ["geometry", "nurbs", "bspline", "periodic", "extrusion"]

[volume]
value = 3333.3333333333335
exact = "the section's area times 10: the square 400 less, at each corner, a third of the triangle of 50 the parabolic arc cuts off (a parabolic segment is ⅔ of its triangle), 400 − 4·50/3 = 1000/3; times 10, 10000/3"
tex = '10\left(400 - 4\cdot\tfrac{50}{3}\right) = \tfrac{10000}{3}'

[area]
value = 1315.9567627227589
exact = "the ends 2·1000/3, + the side 10 times the section's perimeter, four parabolic arcs each 10(1 + ln(1 + √2)/√2) long"
tex = '\tfrac{2000}{3} + 400\left(1 + \frac{\ln\left(1 + \sqrt 2\right)}{\sqrt 2}\right)'
+++

A prism standing on the xy plane, from z = 0 to z = 10, whose section is a closed, periodic
B-spline curve: a square with rounded corners, rounded all the way to the middle of each side.

The curve is the periodic (unclamped, closed) uniform quadratic B-spline, non-rational, whose
control polygon is the square of corners (10, 10), (−10, 10), (−10, −10) and (10, −10) (in the
plane z = 0, taken in that order and closing back on the first). Its knots are evenly spaced and
it has no end: it is one smooth curve, continuous in its tangent everywhere, made of four
parabolic arcs, each running from the middle of one side to the middle of the next, (10, 0) to
(0, 10) and so on, tangent to the sides there, with the square's corner as its control point.

The prism's side is that curve swept straight up: a periodic B-spline surface, closed round on
itself with no seam a kernel must place anywhere in particular. Its ends are flat.
