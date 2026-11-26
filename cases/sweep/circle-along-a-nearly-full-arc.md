+++
title = "A circle swept along 350° of an arc"
tags = ["sweep", "planar-path", "adversarial"]
origin = "adversarial: sweep/circle-along-arc carried round 350° instead of 90°"

[volume]
value = 767.6358978625057
exact = "πr²·Rθ, r = 2, R = 10, θ = 35π/18 (350°)"
tex = '\pi r^2\cdot R\theta,\quad r = 2,\ R = 10,\ \theta = \tfrac{35\pi}{18}'

[area]
value = 792.768639091224
exact = "2πr·Rθ + 2πr²"
tex = '2\pi r\cdot R\theta + 2\pi r^2'
+++

A circle of radius 2 swept along 350° of an arc, `sweep/circle-along-arc` carried nearly all the
way round. The arc lies in the xy plane about the z axis, with radius 10, and runs anticlockwise
seen from +z from (10, 0, 0), through (0, 10, 0), (−10, 0, 0) and (0, −10, 0), to
(10 cos 350°, 10 sin 350°, 0). The section stays square to the path, and the ends are flat. They
face each other across a gap of 10° between the planes through the z axis at 350° and at 0°.

The solid is the part of the solid torus about the z axis, its tube's middle circle of radius 10
and the tube of radius 2, whose points lie at angles from 0° to 350° about the z axis.

**As a set:**

$$\{\,(\rho - 10)^2 + z^2 \le 2^2,\ 0^\circ \le \varphi \le 350^\circ\,\}$$
