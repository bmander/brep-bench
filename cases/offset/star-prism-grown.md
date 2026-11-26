+++
title = "A five-pointed star prism grown by a half"
tags = ["offset", "grow", "planar", "concave-edge", "corner", "oblique", "adversarial"]
origin = "adversarial: offset/l-block-grown with five concave edges at an oblique angle"

[volume]
value = 535.278640200946
exact = "t(A + Pd + kd²) + 2(Ad + πPd²/4 + 2kd³/3), A = 250 sin 36°, P = 10e, e = √(125 − 100 cos 36°), k = 5π/2 − 5γ − 5 tan(54° − γ), γ = asin(5 sin 36°/e), t = 2, d = 1/2"
tex = 't\,(A + Pd + kd^2) + 2\left(Ad + \tfrac{\pi}{4}Pd^2 + \tfrac{2}{3}kd^3\right),\quad A = 250\sin 36^\circ,\ P = 10e,\ e = \sqrt{125 - 100\cos 36^\circ},\ k = \tfrac{5\pi}{2} - 5\gamma - 5\tan(54^\circ - \gamma),\ \gamma = \arcsin\tfrac{5\sin 36^\circ}{e},\ t = 2,\ d = \tfrac12'

[area]
value = 539.8160064422236
exact = "t(P + 2kd) + 2(A + πPd/2 + 2kd²)"
tex = 't\,(P + 2kd) + 2\left(A + \tfrac{\pi}{2}Pd + 2kd^2\right)'
+++

A five-pointed star prism: its outline in the xy plane is the polygon through ten points around
the origin, alternately 10 and 5 from it, the first at 10 on the +y axis, (0, 10), and each next
36° further anticlockwise, so that the points of the star are 10 from the origin at 90°, 162°,
234°, 306° and 18°, and its inside corners 5 from it at 126°, 198°, 270°, 342° and 54°. It is
extruded 2 along +z. Grow it by 1/2: the solid of every point within 1/2 of the prism.

Each face moves 1/2 outward. Each convex edge becomes a part of a cylinder of radius 1/2 about it,
and each corner where three convex edges meet a part of a ball. The ten sides are e long. At each
point of the star they meet at 2γ, about 52.5°, so the vertical edge there is rounded by a
cylinder turning through 180° − 2γ; at each inside corner they meet at 2(144° − γ),
about 235.5°, and the concave vertical edge there stays sharp: the two sides beside it, moved
out, meet along a vertical line. Where such an edge meets the top or the bottom face, the rounds
of the two top or bottom edges beside it meet in a sharp concave crease.

The grown prism's section at any height from z = 0 to z = 2 is the outline grown by 1/2: its
sides moved out 1/2, its points rounded to radius 1/2 and its inside corners sharp, of area
A + Pd + kd², where A and P are the outline's area and perimeter, and k counts at each point
half the angle its rounding turns through, (π − 2γ)/2, and takes off at each inside corner the
kite that the two sides beside it both cover, tan(54° − γ). Above the top, at height u
(0 ≤ u ≤ 1/2), the section is the outline grown by √(1/4 − u²) in the same way, and likewise
below the bottom.

**As a set:**

$$\bigl(S \times [0,2]\bigr) \oplus B_{1/2},\quad S = \operatorname{poly}\bigl(p_0, \dots, p_9\bigr),\ p_i = \rho_i\,(\cos(90^\circ + 36^\circ i),\ \sin(90^\circ + 36^\circ i)),\ \rho_i = 10 \text{ for even } i,\ 5 \text{ for odd } i$$
