+++
title = "A square bar bent along a cubic Bézier curve"
tags = ["spline", "sweep", "planar-path", "bezier", "offset"]

[volume]
value = 16000.0
exact = "A·L = 20²·40: the section is centred on a planar path and stays square to it, below the least radius of curvature (15), so it holds A per unit length (Pappus). The path is a Pythagorean-hodograph cubic, |c′(t)| = 6((3 − 2t)² + (1 + t)²), so L = 2·(10 + 5 + 5) = 40 exactly"
tex = 'AL = 20^2\cdot 40,\quad L = \int_0^1 6\left((3 - 2t)^2 + (1 + t)^2\right)dt = 2(10 + 5 + 5) = 40'

[area]
value = 4000.0
exact = "8hL + 2(2h)², h = 10: the top and bottom are flat strips of width 2h centred on the path (area 2hL each); the two upright sides are swept by the path's offsets at ±h, of lengths L ∓ h·(π/2), which add to 2L, each 2h tall; the ends are squares. 1600 + 1600 + 800"
tex = '8hL + 2(2h)^2 = 1600 + 1600 + 800,\quad h = 10'
+++

A bar of square section, 20 × 20, bent along a planar cubic Bézier curve in the xy plane. The
curve's four control points are (0, 0, 0), (16, 12, 0), (18, 26, 0) and (12, 34, 0): it leaves
the origin along (4, 3, 0) and bends steadily to the left, a quarter turn in all, to arrive at
(12, 34, 0) heading along (−3, 4, 0). It bends tightest at its end, where its radius of
curvature is 15.

At every point of the curve the section is the square of side 20 centred on the curve, in the
plane square to the curve there, with two of its sides horizontal (at z = −10 and z = 10) and
two upright. So the bar's top and bottom are flat, and its two upright sides are traced by the
curve's offsets 10 to its left and 10 to its right in the xy plane. The two ends are flat
squares, square to the curve at its ends.

On the inside of the bend the upright side bends sharply where the bar ends: its radius of
curvature there is 15 − 10 = 5. It never folds over itself.
