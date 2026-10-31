+++
title = "A drilled plate grown by 1"
tags = ["offset", "grow", "analytic", "torus", "saddle"]

[volume]
value = 2781.0247397391663
exact = "V₀ − π(r − d)²c − 2π(r²d − πrd²/2 + 2d³/3), V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 4, r = 3, d = 1"
tex = 'V_0 - \pi (r - d)^2 c - 2\pi\left(r^2 d - \tfrac{\pi}{2}rd^2 + \tfrac{2}{3}d^3\right),\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad a = b = 20,\ c = 4,\ r = 3,\ d = 1'

[area]
value = 1449.3945946152583
exact = "A₀ − 2πr² + 2π(r − d)c + 2·2π(πrd/2 − d²), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd²"
tex = 'A_0 - 2\pi r^2 + 2\pi (r - d)\,c + 2\cdot 2\pi\left(\tfrac{\pi}{2}rd - d^2\right),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2'
+++

The plate from (−10, −10, 0) to (10, 10, 4), with a hole of radius 3 drilled through it along the
z axis. Grow it by 1: the solid of every point within 1 of the drilled plate.

The plate grows as a box does: each face moves 1 outward, each outer edge becomes a quarter of a
cylinder of radius 1, each corner an eighth of a ball. The hole narrows to radius 2, and each of
its rims becomes the inner quarter of a torus's tube: a tube of radius 1 about the rim's circle,
of radius 3, on the side towards the axis, a saddle. So the hole flares from radius 2 out to
radius 3 at the grown plate's faces, z = −1 and z = 5, meeting them tangentially.
