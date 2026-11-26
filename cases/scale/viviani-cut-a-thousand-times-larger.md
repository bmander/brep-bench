+++
title = "A ball bored by a cylinder tangent inside it, a thousand times larger"
tags = ["tangent", "boolean", "cut", "ssi", "singular-curve", "large", "scale", "adversarial"]
origin = "adversarial: tangent/viviani-cut a thousand times larger"

[volume]
value = 2983283991282.0845
exact = "4πR³/3 − V₀ = R³(2π/3 + 8/9), R = 10000, V₀ = 2R³(π − 4/3)/3 the common part (tangent/viviani-common)"
tex = '\tfrac{4\pi R^3}{3} - V_0 = R^3\left(\tfrac{2\pi}{3} + \tfrac{8}{9}\right),\quad V_0 = \tfrac{2R^3}{3}\left(\pi - \tfrac{4}{3}\right),\ R = 10000'

[area]
value = 1428318530.7179587
exact = "2πR² + 8R²: the sphere outside the cylinder, 4πR² − 2R²(π − 2), and the wall inside the ball, 4R², R = 10000 (tangent/viviani-common)"
tex = '\left(4\pi R^2 - 2R^2(\pi - 2)\right) + 4R^2 = 2\pi R^2 + 8R^2,\quad R = 10000'
+++

The ball and the bore of `tangent/viviani-cut`, a thousand times larger: a ball 20 m across, if a
unit is a millimetre.

A ball of radius 10000 centered on the origin, bored by a solid cylinder of radius 5000 whose axis
is the line x = 5000, y = 0, parallel to the z axis, running from z = −12000 to z = 12000: the ball
less the cylinder.

The cylinder's radius is half the ball's and its wall passes through the ball's center, so it
touches the ball's sphere from inside at one point, (10000, 0, 0), where the two surfaces are
tangent. Above and below that point the cylinder comes out through the sphere, and its ends, at
z = ±12000, are outside the ball.

The sphere and the wall meet in Viviani's curve, (10000 cos²t, 10000 cos t sin t, 10000 sin t): a
figure eight on the sphere that crosses itself at (10000, 0, 0), and passes through (0, 0, 10000)
and (0, 0, −10000). Around the crossing point what is left is two slivers, one toward +y and one
toward −y, each between the wall and the sphere, thinning to nothing as they reach it; above and
below it nothing is left. The result is one solid, pinched to that point: the slivers touch there
and nowhere else, and its surface is not a manifold at that point.

If the ball you make has poles and a seam, keep them off the curve and away from its crossing:
put the poles on the y axis, at (0, ±10000, 0), and the seam through (−10000, 0, 0). If the cylinder
you make has a seam, put it on the line x = 0, y = 0, the side of its wall away from (10000, 0, 0).

**As a set:**

$$B_{10000} \setminus \{\,(x-5000)^2 + y^2 \le 5000^2,\ |z| \le 12000\,\}$$
