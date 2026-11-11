+++
title = "A cone frustum shelled open at both ends, the cavity closing to a point below its top"
tags = ["shell", "inward", "analytic", "cone", "apex", "degenerate"]

[volume]
value = 238.6383232097934
exact = "πH(R² + Rr + r²)/3 − πa²b/3 = 84π − 8.0390625π = 75.9609375π: R = 6, r = 3, H = 4; the cavity a cone a = 2.625 across its mouth, b = 3.5 deep"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) - \tfrac{\pi a^2 b}{3} = 75.9609375\pi,\quad R = 6,\ r = 3,\ H = 4,\ a = 2.625,\ b = 3.5'

[area]
value = 297.1750300755095
exact = "π(R + r)s + π(R² − a²) + πr² + πac = 94.59375π: s = 5 the frustum's side, c = 4.375 the cavity's"
tex = '\pi\left(R + r\right)s + \pi\left(R^2 - a^2\right) + \pi r^2 + \pi a c = 94.59375\pi,\quad s = 5,\ c = 4.375'
+++

A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of
radius 3 at z = 4, so its side is 5 long and leans in at the half-angle α with sin α = 3/5; the
side, run on, would close at its apex (0, 0, 8). Shell it inward to a wall 2.7 thick, open at both
its base, z = 0, and its top, z = 4. The outside of the wall is its side, unmoved, and the inside is
the side moved 2.7 inward along its normal, onto the surface offset 2.7 from it. The moved faces
are extended or trimmed where they meet one another, and run on until they meet the surface of an
opening, which is left as a rim between the outside and the inside.

The side, moved 2.7 inward, is the cone with the same half-angle whose apex is 2.7/sin α = 4.5
lower, at (0, 0, 3.5). It meets the base's plane in the circle of radius 2.625, and there leaves a
rim, the annulus between radii 2.625 and 6. Run on, it closes to its apex 0.5 below the top, so it
never meets the top: no rim is made there, and the top stays as it was. The cavity is the cone from
the disk of radius 2.625 at z = 0 to the point (0, 0, 3.5), its side 4.375 long, open at the base
only. The result is the frustum less that cone. A kernel that cannot tell that the top is never
reached may refuse; one that opens the top anyway, or runs the cavity up through it, is wrong.

**As a set:**

$$\{\,\rho \le \tfrac34(8 - z),\ 0 \le z \le 4\,\} \setminus \{\,\rho \le \tfrac34(3.5 - z)\,\}$$
