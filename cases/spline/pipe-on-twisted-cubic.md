+++
title = "A pipe along a twisted cubic Bézier curve"
tags = ["spline", "sweep", "space-path", "bezier", "torsion"]

[volume]
value = 8042.47719318987
exact = "πr²L = 2560π, r = 8, L = 40: a tube whose radius is below the path's least radius of curvature (10.28) and whose sections never meet holds πr² per unit length whatever the path's torsion. The path is a spatial Pythagorean-hodograph cubic, |c′(t)| = 6(10(1 − t)² + 2t(1 − t) + 9t²), so L = 2·(10 + 1 + 9) = 40 exactly"
tex = '\pi r^2 L = 2560\pi,\quad r = 8,\ L = \int_0^1 6\left(10(1 - t)^2 + 2t(1 - t) + 9t^2\right)dt = 40'

[area]
value = 2412.743157956961
exact = "2πrL + 2πr² = 640π + 128π = 768π: the wall holds 2πr per unit length (torsion only slides the section round in its own plane), and the two ends are discs"
tex = '2\pi r L + 2\pi r^2 = 640\pi + 128\pi = 768\pi'
+++

A pipe of circular section, radius 8, swept along a cubic Bézier curve that does not lie in a
plane. The curve's four control points are (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16):
it leaves the origin along (3, 4, 0), swings round to the right and rises out of the xy plane to
end at (24, 0, 16), arriving along (−1, −4, 8). Its torsion never vanishes, and its radius of
curvature is least, 10.28, a little past the middle of the curve.

At every point of the curve the section is the disc of radius 8 centred on the curve, in the
plane square to the curve there. Since the section is a disc, how it turns about the curve does
not change the pipe. The pipe's two ends are flat discs, square to the curve at its two ends.

On the inside of the tightest part of the bend the wall's radius of curvature is only about
2.3; the wall never folds over itself and no two sections meet.
