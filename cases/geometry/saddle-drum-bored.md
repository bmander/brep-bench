+++
title = "A drum under a saddle, bored through"
tags = ["geometry", "nurbs", "bspline", "trimmed", "hole", "saddle", "boolean"]

[volume]
value = 1727.8759594743863
exact = "∫∫(10 + xy/8) over the annulus 3 ≤ ρ ≤ 8 = 550π"
tex = '\iint_{3 \le \rho \le 8}\left(10 + \tfrac{xy}{8}\right)dA = 550\pi'

[area]
value = 1079.7764991189633
exact = "the floor 55π + the outer side 160π + the bore's wall 2π·3·10 = 60π + the top ∫∫√(1 + ρ²/64) dA over the annulus = (128π/3)(2√2 − 73√73/512)"
tex = '55\pi + 160\pi + 60\pi + \tfrac{128\pi}{3}\left(2\sqrt 2 - \tfrac{73\sqrt{73}}{512}\right)'
+++

The drum of `geometry/saddle-drum`, bored through along its axis: every point with
9 ≤ x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8.

The saddle is the same B-spline surface: the bilinear patch of degree 1 each way, non-rational,
whose four control points are (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and
(10, 10, 22.5). The drum's top is the part of it between the circles of radius 3 and 8 about the z
axis, a B-spline face trimmed by an outer loop and with a hole. The bore, of radius 3, runs from
the floor to the saddle, which it meets at heights from 9.4375 to 10.5625. If you cut it with a
cylinder, make it run past the floor and the saddle.
