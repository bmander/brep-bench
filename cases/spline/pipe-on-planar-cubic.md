+++
title = "A fat pipe round a cubic Bézier bend"
tags = ["spline", "sweep", "planar-path", "bezier", "near-cusp"]

[volume]
value = 65345.12719466769
exact = "πr²L, r = 20, L = 52: a tube whose radius is below the path's least radius of curvature (24) and whose sections never meet holds πr² per unit length (Pappus/Weyl). The path is a Pythagorean-hodograph cubic, |c′(t)| = 12(4 + (2t − 1)²), so L = ∫₀¹ |c′| dt = 12·(4 + 1/3) = 52 exactly"

[area]
value = 9047.786842338604
exact = "2πrL + 2πr² = 2080π + 800π = 2880π: the wall holds 2πr per unit length for the same reason, and the two ends are discs"
+++

A pipe of circular section, radius 20, swept along a planar cubic Bézier curve in the xy plane.
The curve's four control points are (0, 0, 0), (12, −16, 0), (32, −16, 0) and (44, 0, 0): it
leaves the origin heading down and to the right, along (3, −4, 0), dips to y = −12 at x = 22 and
climbs back to (44, 0, 0), arriving along (3, 4, 0). The curve is a polynomial cubic, not a
circular arc: its radius of curvature runs from 24 at its lowest point to 37.5 at its ends.

At every point of the curve the section is the disc of radius 20 centred on the curve, in the
plane square to the curve there. The pipe's two ends are flat discs, square to the curve at its
two ends.

The radius is close to the curve's least radius of curvature, so on the inside of the bend the
wall curves tightly, with a radius of curvature of only 24 − 20 = 4 at the lowest point; it
never folds over itself and no two sections meet.
