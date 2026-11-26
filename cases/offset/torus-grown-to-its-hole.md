+++
title = "A torus grown just to its hole"
tags = ["offset", "grow", "analytic", "torus", "horn", "adversarial"]
origin = "adversarial: offset/torus-grown-past-its-hole grown by 2 instead of 3, so the tube closes the hole exactly"

[volume]
value = 1263.309363339438
exact = "2π²Rs², s = r + d = R: 2π²R³, R = 4, r = 2, d = 2"
tex = '2\pi^2 R s^2,\quad s = r + d = R:\ 2\pi^2 R^3,\quad R = 4,\ r = 2,\ d = 2'

[area]
value = 631.6546816697189
exact = "4π²Rs = 4π²R²"
tex = '4\pi^2 R s = 4\pi^2 R^2'
+++

A solid torus about the z axis, centered on the origin, whose tube has radius 2 about a middle
circle of radius 4, as in `offset/torus-grown-past-its-hole`. Grow it by 2: the solid of every
point within 2 of the torus.

That is every point within 4 of the middle circle. The grown tube's radius, 4, equals the middle
circle's, so the grown torus just closes its hole: it is the horn torus of `geometry/horn-torus`,
whose surface comes to the z axis at one point, the origin, where it meets itself like two cones
tip to tip. Its section through the axis, on one side of it, is the disk of radius 4 about (4, 0),
which touches the axis at the origin.

**As a set:**

$$\{\,(\rho - 4)^2 + z^2 \le 2^2\,\} \oplus B_{2} = \{\,(\rho - 4)^2 + z^2 \le 4^2\,\}$$
