+++
title = "A bored cone frustum 3.1 times the size, shelled open at its top, the cavity a ring sealed below it"
tags = ["shell", "inward", "analytic", "cone", "void", "degenerate", "adversarial"]
origin = "adversarial: shell/bored-frustum-sealed-below-its-opening with every length times 3.1"

[volume]
value = 6156.220283545708
exact = "πH(R² + Rr + r²)/3 − πb²H − 2πρ̄A = 592πs³/9: R = 18.6, r = 9.3, H = 12.4, b = 3.1, s = 3.1; the cavity the triangle (6.2, 3.1), (12.4, 3.1), (6.2, 11s/3) turned about the axis, A = 8s²/3, ρ̄ = 8s/3"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) - \pi b^2 H - 2\pi\bar\rho A = \tfrac{592\pi s^3}{9},\quad s = 3.1,\ A = \tfrac{8s^2}{3},\ \bar\rho = \tfrac{8s}{3}'

[area]
value = 4186.444482271711
exact = "π(R + r)ℓ + π(R² − b²) + π(r² − b²) + 2πbH + π((4s)² − (2s)²) + 2π·2s·8s/3 + π(4s + 2s)·10s/3 = 416πs²/3: ℓ = 15.5 the side's length, s = 3.1"
tex = '\pi(R + r)\ell + \pi(R^2 - b^2) + \pi(r^2 - b^2) + 2\pi bH + \pi\left((4s)^2 - (2s)^2\right) + 2\pi\cdot 2s\cdot\tfrac{8s}{3} + \pi(4s + 2s)\tfrac{10s}{3} = \tfrac{416\pi s^2}{3},\quad s = 3.1'
+++

The bored frustum of `shell/bored-frustum-sealed-below-its-opening` with every length 3.1 times
as long. A solid cone frustum on the z axis, its base the disk of radius 18.6 at z = 0 and its top
the disk of radius 9.3 at z = 12.4, so its side is 15.5 long and leans in at the half-angle α with
sin α = 3/5, bored through along the axis by a hole of radius 3.1. Its top is the annulus between
radii 3.1 and 9.3 at z = 12.4. Shell it inward to a wall 3.1 thick, open at its top. The outside of
the wall is its side, its base and its bore, unmoved, and the inside is each of them moved 3.1
inward along its normal, onto the surface offset 3.1 from it. The moved faces are extended or
trimmed where they meet one another, and run on until they meet the surface of an opening, which is
left as a rim between the outside and the inside.

The side, moved 3.1 inward, is the cone ρ = 14.725 − 3z/4; the base, moved 3.1 up, is the plane
z = 3.1; the bore, moved 3.1 outward from the axis, is the cylinder of radius 6.2. The moved side,
that cone run on, meets the moved base in the circle of radius 12.4 and the moved bore in the circle
of radius 6.2 at z = 34.1/3 ≈ 11.367, about 1.03 below the top. So the moved faces close on one
another before the opening, the cavity never reaches it, and no rim is made: the top stays as it
was, and the cavity is the closed ring bounded by the moved side, the moved bore and the moved base,
its section the triangle (6.2, 3.1), (12.4, 3.1), (6.2, 34.1/3) in (ρ, z), its sides 6.2, 24.8/3
and 31/3. The result is the bored frustum less that ring, one solid bounded by two shells. A kernel
that cannot tell that the opening is never reached may refuse; one that removes the top anyway, or
runs the cavity up through it, is wrong.

**As a set:**

$$\{\,3.1 \le \rho \le \tfrac34(24.8 - z),\ 0 \le z \le 12.4\,\} \setminus \{\,6.2 \le \rho \le 14.725 -
\tfrac34 z,\ z \ge 3.1\,\}$$
