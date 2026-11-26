+++
title = "Viviani's solid a thousand times larger: a ball and a cylinder tangent inside it, their common part"
tags = ["tangent", "boolean", "common", "ssi", "singular-curve", "large", "scale", "adversarial"]
origin = "adversarial: tangent/viviani-common a thousand times larger"

[volume]
value = 1205506213504.3066
exact = "2R³(π − 4/3)/3, R = 10000: over the cylinder's disk r ≤ R cos θ, ∫∫ 2√(R² − r²) r dr dθ = (2R³/3)∫(1 − |sin θ|³) dθ"
tex = '\tfrac{2R^3}{3}\left(\pi - \tfrac{4}{3}\right),\quad R = 10000'

[area]
value = 628318530.7179586
exact = "2πR², R = 10000: the ball's sphere inside the cylinder, 2R²(π − 2), and the cylinder's wall inside the ball, ∫ 2R|sin(φ/2)|·(R/2) dφ over a turn = 4R²"
tex = '2R^2(\pi - 2) + 4R^2 = 2\pi R^2,\quad R = 10000'
+++

The ball and the cylinder of `tangent/viviani-common`, a thousand times larger: a ball 20 m across,
if a unit is a millimetre.

A ball of radius 10000 centered on the origin, and a solid cylinder of radius 5000 whose axis is the
line x = 5000, y = 0, parallel to the z axis, running from z = −12000 to z = 12000. Make the solid
common to both.

The cylinder's radius is half the ball's and its wall passes through the ball's center, so it
touches the ball's sphere from inside at one point, (10000, 0, 0), where the two surfaces are
tangent. Elsewhere the wall leaves the ball: its ends, at z = ±12000, are outside it. The common
part is the stretch of the cylinder inside the ball, capped above and below by the sphere.

The sphere and the wall meet in Viviani's curve, (10000 cos²t, 10000 cos t sin t, 10000 sin t): a
figure eight on the sphere that crosses itself at (10000, 0, 0), and passes through (0, 0, 10000)
and (0, 0, −10000). Around the crossing point the surface is four wedges, alternating: the
cylinder's wall toward +y and toward −y, and the sphere above and below. The solid is convex, and
its surface is a manifold there.

If the ball you make has poles and a seam, keep them off the curve and away from its crossing:
put the poles on the y axis, at (0, ±10000, 0), and the seam through (−10000, 0, 0). If the cylinder
you make has a seam, put it on the line x = 0, y = 0, the side of its wall away from (10000, 0, 0).

**As a set:**

$$B_{10000} \cap \{\,(x-5000)^2 + y^2 \le 5000^2,\ |z| \le 12000\,\}$$
