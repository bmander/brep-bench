+++
title = "A parabolic trough, a B-spline sheet, thickened on its concave side"
tags = ["thicken", "sheet", "nurbs", "bspline", "concave"]

[volume]
value = 83.96950434173104
exact = "h(tL − t²θ/2), L = 4√2 + 4 asinh 1 the parabola's length, θ = π/2 the angle its tangent turns, t = 1, h = 10"
tex = 'h\left(tL - \tfrac{1}{2}t^2\theta\right),\quad L = 4\sqrt{2} + 4\operatorname{asinh} 1,\ \theta = \tfrac{\pi}{2},\ t = 1,\ h = 10'

[area]
value = 204.7329095518083
exact = "h(L + (L − tθ)) + 2(tL − t²θ/2) + 2th"
tex = 'h\left(L + (L - t\theta)\right) + 2\left(tL - \tfrac{1}{2}t^2\theta\right) + 2th'
+++

A sheet: the parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, made as a B-spline
surface: of degree 2 in u and 1 in v, with no interior knots (u and v each from 0 to 1, their
end knots of full multiplicity), all weights 1, and control points (−4, y, 2), (0, y, −2) and
(4, y, 2) in u, for y = 0 and y = 10 in v. Its quadratic Bézier is exactly the parabola, with x
running evenly in u. It is one face, bounded by two parabolic arcs, at y = 0 and y = 10, and two
straight lines, at x = −4 and x = 4.

Thicken it by 1 to its concave side, upward toward the parabola's focus (0, y, 2). The parabola's
radius of curvature is at least 4 (at its vertex), so the moved face does not cross itself. The
thickened sheet is the parabola's strip between it and its offset curve 1 toward the focus,
extruded from y = 0 to y = 10. Its sides at x = ±4 are flat strips, 1 by 10, along the parabola's
normals there, which lean in at 45°; its ends are the strip itself, at y = 0 and y = 10.
