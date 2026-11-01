+++
title = "A drum under a saddle"
tags = ["geometry", "nurbs", "bspline", "trimmed", "saddle", "boolean"]

[volume]
value = 2010.6192982974676
exact = "∫∫(10 + xy/8) over the disc of radius 8 = 640π (xy integrates to 0)"
tex = '\iint_{\rho \le 8}\left(10 + \tfrac{xy}{8}\right)dA = 640\pi'

[area]
value = 948.8014785737964
exact = "the floor 64π + the side 2π·8·10 = 160π (the saddle's rise and fall cancel round it) + the top ∫∫√(1 + ρ²/64) dA = (128π/3)(2√2 − 1)"
tex = '64\pi + 160\pi + \tfrac{128\pi}{3}\left(2\sqrt 2 - 1\right)'
+++

A round drum of radius 8 about the z axis, its floor flat at z = 0 and its top the saddle
z = 10 + xy/8: every point with x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8.

Make the saddle as a B-spline surface: the bilinear patch of degree 1 each way, non-rational,
whose four control points are (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and
(10, 10, 22.5), the corners of the square −10 ≤ x, y ≤ 10 at the heights of the saddle there.
That patch is exactly the saddle over the whole square (a bilinear patch is a hyperbolic
paraboloid). The drum's top is the part of it inside the circle of radius 8, trimmed by the drum's
side: it rises to 14 above the diagonal x = y and falls to 6 above x = −y.
