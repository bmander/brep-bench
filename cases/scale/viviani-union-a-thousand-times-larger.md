+++
title = "A ball and a cylinder tangent inside it, united, a thousand times larger"
tags = ["tangent", "boolean", "union", "ssi", "singular-curve", "large", "scale", "adversarial"]
origin = "adversarial: tangent/viviani-union a thousand times larger"

[volume]
value = 4868239583435.96
exact = "4πR³/3 + π(R/2)²·2.4R − V₀ = R³(19π/15 + 8/9), R = 10000, V₀ = 2R³(π − 4/3)/3 the common part (tangent/viviani-common)"
tex = '\tfrac{4\pi R^3}{3} + 2.4R\,\pi\left(\tfrac{R}{2}\right)^2 - V_0 = R^3\left(\tfrac{19\pi}{15} + \tfrac{8}{9}\right),\quad V_0 = \tfrac{2R^3}{3}\left(\pi - \tfrac{4}{3}\right),\ R = 10000'

[area]
value = 1539380400.2589986
exact = "4.9πR²: the sphere outside the cylinder, 4πR² − 2R²(π − 2), the wall outside the ball, 2π(R/2)·2.4R − 4R², and the two end disks, 2π(R/2)², R = 10000 (tangent/viviani-common)"
tex = '\left(4\pi R^2 - 2R^2(\pi - 2)\right) + \left(2\pi\tfrac{R}{2}\cdot 2.4R - 4R^2\right) + 2\pi\left(\tfrac{R}{2}\right)^2 = 4.9\pi R^2,\quad R = 10000'
+++

The ball and the cylinder of `tangent/viviani-union`, a thousand times larger: a ball 20 m across,
if a unit is a millimetre.

A ball of radius 10000 centered on the origin, and a solid cylinder of radius 5000 whose axis is the
line x = 5000, y = 0, parallel to the z axis, running from z = −12000 to z = 12000. Unite them.

The cylinder's radius is half the ball's and its wall passes through the ball's center, so it
touches the ball's sphere from inside at one point, (10000, 0, 0), where the two surfaces are
tangent. Above and below that point the cylinder comes out through the sphere, and its ends, at
z = ±12000, are outside the ball.

The sphere and the wall meet in Viviani's curve, (10000 cos²t, 10000 cos t sin t, 10000 sin t): a
figure eight on the sphere that crosses itself at (10000, 0, 0), and passes through (0, 0, 10000)
and (0, 0, −10000). Around the crossing point the surface is four wedges, alternating: the sphere
toward +y and toward −y, and the cylinder's wall above and below. The solid's surface is a manifold
there.

If the ball you make has poles and a seam, keep them off the curve and away from its crossing:
put the poles on the y axis, at (0, ±10000, 0), and the seam through (−10000, 0, 0). If the cylinder
you make has a seam, put it on the line x = 0, y = 0, the side of its wall away from (10000, 0, 0).

**As a set:**

$$B_{10000} \cup \{\,(x-5000)^2 + y^2 \le 5000^2,\ |z| \le 12000\,\}$$
