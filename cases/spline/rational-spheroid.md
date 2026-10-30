+++
title = "A spheroid turned from a weighted quadratic B-spline"
tags = ["spline", "revolve", "rational", "weights", "pole", "knots"]

[volume]
value = 1507.9644737231006
exact = "4πab²/3 = 480π, a = 10, b = 6: the weighted spline is exactly half the ellipse x²/36 + z²/100 = 1, so the solid is a prolate spheroid"

[area]
value = 663.1722477458867
exact = "2πb²(1 + (a/(be))·asin e), e = √(1 − b²/a²) = 4/5: 72π + 150π·asin(4/5)"
+++

A solid made by turning a plane region a full turn about the z axis. The region lies in the xz
plane, on the side x ≥ 0, between the segment of the z axis from (0, 0, −10) to (0, 0, 10) and a
rational quadratic B-spline curve joining the same two points.

The curve has five control points, (0, 0, −10), (6, 0, −10), (6, 0, 0), (6, 0, 10) and
(0, 0, 10), with weights 1, √2/2, 1, √2/2 and 1, and the clamped knot vector
(0, 0, 0, ½, ½, 1, 1, 1). The double knot at ½ splits it into two rational quadratic Bézier
segments, each through three of the control points (the middle one, (6, 0, 0), shared). Each
segment is a quarter of the ellipse x²/36 + z²/100 = 1: the curve leaves the axis at the bottom
square to it, swells to x = 6 at z = 0 and returns to the axis square to it at the top. Without
the weights the segments would be parabolas.

So the solid is a smooth egg-like spheroid, 12 wide and 20 tall, with one face whose two poles
lie on the axis.
