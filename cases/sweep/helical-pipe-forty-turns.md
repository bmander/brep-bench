+++
title = "A helical pipe of forty turns"
tags = ["sweep", "helix", "spring", "adversarial"]
origin = "adversarial: sweep/helical-pipe with forty turns instead of three"

[volume]
value = 7920.644067107493
exact = "πr²L, L = n√((2πR)² + p²), r = 1, R = 10, p = 5, n = 40"
tex = '\pi r^2 L,\quad L = n\sqrt{(2\pi R)^2 + p^2},\ r = 1,\ R = 10,\ p = 5,\ n = 40'

[area]
value = 15847.571319522167
exact = "2πrL + 2πr²"
tex = '2\pi r L + 2\pi r^2'
+++

A pipe of circular section, radius 1, following a helix about the z axis, as in
`sweep/helical-pipe`, but forty turns long: a coil spring. The helix has radius 10 and rises 5
per turn. It starts at (10, 0, 0), turns right-handed (from +x toward +y) and makes forty whole
turns, ending at (10, 0, 200). The section stays square to the path, and the pipe's ends are
flat.

The coils stand 5 apart along the axis and the pipe is 2 across, so neighbouring coils do not
touch, and the helix's radius of curvature, (10² + (5/2π)²)/10 ≈ 10.06, is far more than the
pipe's radius. So the pipe's volume is its section's area times the length of its path, and the
area of its side the section's perimeter times that length.

**As a set:**

$$\bigl\{\,c(\theta) + u : 0 \le \theta \le 40 \cdot 360^\circ,\ u \perp c'(\theta),\ \lVert u \rVert \le
1\,\bigr\}, \quad c(\theta) = \bigl(10\cos\theta,\ 10\sin\theta,\
5\,\tfrac{\theta}{360^\circ}\bigr)$$
