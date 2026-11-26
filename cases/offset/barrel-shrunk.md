+++
title = "A ball cut by two planes shrunk by 1"
tags = ["offset", "shrink", "sphere", "planar", "analytic", "adversarial"]
origin = "adversarial: offset/box-shrunk with a ball's zone between two planes in place of the box"

[volume]
value = 184.3067690106012
exact = "∫ π((R − d)² − z²) dz over |z| ≤ h − d = 176π/3, R = 5, h = 3, d = 1"
tex = '\int_{-(h - d)}^{h - d} \pi\left((R - d)^2 - z^2\right)dz = \tfrac{176\pi}{3},\quad R = 5,\ h = 3,\ d = 1'

[area]
value = 175.92918860102842
exact = "2π(R − d)·2(h − d) + 2π((R − d)² − (h − d)²) = 56π"
tex = '2\pi (R - d)\cdot 2(h - d) + 2\pi\left((R - d)^2 - (h - d)^2\right) = 56\pi'
+++

A barrel: the ball of radius 5 centered on the origin, cut by the planes z = −3 and z = 3, keeping
the part between them, whose flat ends are disks of radius 4. Shrink it by 1: keep every point of
it at least 1 from its boundary.

A point is at least 1 inside the ball and at least 1 inside each plane exactly when it is in the
ball of radius 4 and between the planes z = −2 and z = 2, so the result is the same kind of barrel:
the ball of radius 4 about the origin cut by z = ±2, its flat ends disks of radius √12. Its edges,
where the sphere meets the ends, are convex and stay sharp.

**As a set:**

$$\bigl(B_5 \cap \{\,|z| \le 3\,\}\bigr) \ominus B_1 = B_4 \cap \{\,|z| \le 2\,\}$$
