+++
title = "A block with a deep blind bore grown past the bore's radius"
tags = ["offset", "grow", "analytic", "torus", "vanishing-face", "adversarial"]
origin = "adversarial: offset/drilled-plate-grown-past-its-hole with the hole made blind and deep"

[volume]
value = 13175.698418612252
exact = "V₀ − 2π(dρ²/2 − ρ/2·(ρq + d²·asin(ρ/d)) + (d³ − q³)/3), V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 10, ρ = 3, d = 4, q = √(d² − ρ²) = √7"
tex = 'V_0 - 2\pi\left(\tfrac{d\rho^2}{2} - \tfrac{\rho}{2}\left(\rho q + d^2\arcsin\tfrac{\rho}{d}\right) + \tfrac{d^3 - q^3}{3}\right),\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad a = b = 20,\ c = 10,\ \rho = 3,\ d = 4,\ q = \sqrt{7}'

[area]
value = 3059.3310498558026
exact = "A₀ − πρ² + 2πd(ρφ − d(1 − cos φ)), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², φ = asin(ρ/d), a = b = 20, c = 10, ρ = 3, d = 4"
tex = 'A_0 - \pi\rho^2 + 2\pi d\left(\rho\varphi - d(1 - \cos\varphi)\right),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2,\quad \varphi = \arcsin\tfrac{\rho}{d},\ a = b = 20,\ c = 10,\ \rho = 3,\ d = 4'
+++

The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
the top, 8 deep, to a flat floor at z = 2. Grow it by 4, more than the bore's radius: the solid
of every point within 4 of the block.

The block grows as a box does: each face moves 4 outward, each edge becomes a quarter of a
cylinder of radius 4, each corner an eighth of a ball. The bore is gone, filled solid: every point
of it is within 3 of its wall. Over the bore's mouth, as over a hole drilled through
(`offset/drilled-plate-grown-past-its-hole`), a point is within 4 of the block only within 4 of
the bore's rim (the floor, 8 down, is too far to matter), so the grown top dips in a dimple: the
inner part of a spindle torus, a tube of radius 4 about the rim's circle of radius 3, whose lowest
point is on the axis √7 ≈ 2.65 above the top face. The bottom stays flat, and there is no cavity
anywhere inside.

**As a set:**

$$\Bigl([-10,10]^2\times[0,10] \setminus \{\,\rho \le 3,\ z \ge 2\,\}\Bigr) \oplus B_{4}$$
