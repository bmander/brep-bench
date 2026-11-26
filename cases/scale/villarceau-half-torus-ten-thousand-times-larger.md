+++
title = "A torus ten thousand times larger cut by a plane tangent to it at two points"
tags = ["tangent", "boolean", "cut", "ssi", "singular-curve", "torus", "large", "scale", "adversarial"]
origin = "adversarial: tangent/villarceau-half-torus ten thousand times larger"

[volume]
value = 444132198049021.14
exact = "π²Rr², R = 50000, r = 30000: half the torus's 2π²Rr², as the turn of a half turn about the z axis followed by the reflection in the xy plane (the point reflection through the origin) maps the torus to itself and the plane to itself, swapping its sides"
tex = '\pi^2 R r^2,\quad R = 50000,\ r = 30000'

[area]
value = 40843824291.2009
exact = "2π²Rr + 2(πR² − L), R = 50000, r = 30000: half the torus's 4π²Rr (by the same symmetry), and the cut face, two disks of radius R whose centers are 2r apart less twice their lens, L = 2R² arccos(r/R) − 2r√(R² − r²)"
tex = '2\pi^2 R r + 2\left(\pi R^2 - L\right),\quad L = 2R^2\arccos\tfrac{r}{R} - 2r\sqrt{R^2 - r^2},\quad R = 50000,\ r = 30000'
+++

The torus and the plane of `tangent/villarceau-half-torus`, ten thousand times larger: a torus
160 m across, if a unit is a millimetre.

A solid torus about the z axis, centered on the origin: the circle through the middle of its tube
has radius 50000, and the tube has radius 30000. Cut it by the plane 3x − 4z = 0, the plane through
the y axis that rises 3 in z for every 4 in x, and keep what lies on and below it: every point of
the torus with z ≤ 3x/4. If you cut with a block or a half-space, make it reach past the torus.

The plane makes the angle arcsin(3/5) with the xy plane, and 3/5 is the tube's radius over the
middle circle's, so it is a bitangent plane: it touches the torus at two points, (32000, 0, 24000)
and (−32000, 0, −24000), on the inner side of the tube, where the surface is saddle-shaped. It cuts
the torus in two circles of radius 50000, Villarceau's circles, centered at (0, 30000, 0) and
(0, −30000, 0) in the plane, which cross each other at those two points. The cut face is what lies
inside one circle and not the other: two crescents, touching at their tips.

The two tangent points are not alike. At (32000, 0, 24000) the torus lies on the kept side of the
plane: around that point the surface is four wedges, alternating, the two crescents' tips and the
torus's surface between them, and the surface is a manifold there. At (−32000, 0, −24000) the torus
lies on the side cut away: around that point what is left is two slivers, between the plane and the
torus's surface, touching at their tips. The result is one solid, a half of the tube from one
crescent around to the other, pinched to that point, where its surface is not a manifold.

If the torus you make has a seam where the tube's section was turned from, put it in the plane
x = 0, on the side y > 0, away from the two tangent points.

**As a set:**

$$\{\,(\rho - 50000)^2 + z^2 \le 30000^2,\ z \le \tfrac34 x\,\}$$
