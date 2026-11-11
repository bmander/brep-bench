+++
title = "A bored cone frustum shelled open at its top, the cavity a ring sealed below it"
tags = ["shell", "inward", "analytic", "cone", "void", "degenerate"]

[volume]
value = 206.6469834361286
exact = "πH(R² + Rr + r²)/3 − πb²H − 2πρ̄A = 84π − 4π − 128π/9 = 592π/9: R = 6, r = 3, H = 4, b = 1; the cavity the triangle (2, 1), (4, 1), (2, 11/3) turned about the axis, A = 8/3, ρ̄ = 8/3"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) - \pi b^2 H - 2\pi\bar\rho A = \tfrac{592\pi}{9},\quad A = \bar\rho = \tfrac83'

[area]
value = 435.63418129778466
exact = "π(R + r)s + π(R² − b²) + π(r² − b²) + 2πbH + π(4² − 2²) + 2π·2·8/3 + π(4 + 2)·10/3 = 96π + 128π/3 = 416π/3: s = 5"
tex = '96\pi + \pi\left(4^2 - 2^2\right) + 2\pi\cdot 2\cdot\tfrac83 + \pi\left(4 + 2\right)\tfrac{10}{3} = \tfrac{416\pi}{3}'
+++

A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of
radius 3 at z = 4, so its side is 5 long and leans in at the half-angle α with sin α = 3/5, bored
through along the axis by a hole of radius 1. Its top is the annulus between radii 1 and 3 at
z = 4. Shell it inward to a wall 1 thick, open at its top. The outside of the wall is its side, its
base and its bore, unmoved, and the inside is each of them moved 1 inward along its normal, onto the
surface offset 1 from it. The moved faces are extended or trimmed where they meet one another, and
run on until they meet the surface of an opening, which is left as a rim between the outside and
the inside.

The side, moved 1 inward, is the cone ρ = 4.75 − 3z/4; the base, moved 1 up, is the plane z = 1; the
bore, moved 1 outward from the axis, is the cylinder of radius 2. The moved side, that cone run on,
meets the moved base in the circle of radius 4 and the moved bore in the circle of radius 2 at z =
11/3, 1/3 below the top. So the moved faces close on one another before the opening, the cavity
never reaches it, and no rim is made: the top stays as it was, and the cavity is the closed ring
bounded by the moved side, the moved bore and the moved base, its section the triangle (2, 1), (4,
1), (2, 11/3) in (ρ, z), its sides 2, 8/3 and 10/3. The result is the bored frustum less that ring,
one solid bounded by two shells. A kernel that cannot tell that the opening is never reached may
refuse; one that removes the top anyway, or runs the cavity up through it, is wrong.

**As a set:**

$$\{\,1 \le \rho \le \tfrac34(8 - z),\ 0 \le z \le 4\,\} \setminus \{\,2 \le \rho \le 4.75 -
\tfrac34 z,\ z \ge 1\,\}$$
