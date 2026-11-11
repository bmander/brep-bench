+++
title = "A cone frustum shelled open at its top, the cavity closing to a point below it"
tags = ["shell", "inward", "analytic", "cone", "apex", "void", "degenerate"]

[volume]
value = 263.592190006798
exact = "πH(R² + Rr + r²)/3 − πa²b/3 = 84π − 0.096π = 83.904π: R = 6, r = 3, H = 4; the cavity a cone a = 0.6 across, b = 0.8 high"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) - \tfrac{\pi a^2 b}{3} = 84\pi - 0.096\pi,\quad R = 6,\ r = 3,\ H = 4,\ a = 0.6,\ b = 0.8'

[area]
value = 285.7592677705276
exact = "π(R² + r²) + π(R + r)s + πa² + πac = 90π + 0.96π = 90.96π: s = 5 the frustum's side, c = 1 the cavity's"
tex = '\pi\left(R^2 + r^2\right) + \pi\left(R + r\right)s + \pi a^2 + \pi a c = 90.96\pi,\quad s = 5,\ c = 1'
+++

A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of
radius 3 at z = 4, so its side is 5 long and leans in at the half-angle α with sin α = 3/5; the
side, run on, would close at its apex (0, 0, 8). Shell it inward to a wall 2.7 thick, open at its
top face, z = 4. The outside of the wall is its side and its base, unmoved, and the inside is each
of them moved 2.7 inward along its normal, onto the surface offset 2.7 from it. The moved faces are
extended or trimmed where they meet one another, and run on until they meet the surface of an
opening, which is left as a rim between the outside and the inside.

The side, moved 2.7 inward, is the cone with the same half-angle whose apex is 2.7/sin α = 4.5
lower, at (0, 0, 3.5); the base, moved 2.7 up, is the plane z = 2.7, which that cone meets in the
circle of radius 0.6. The moved side, that cone run on, closes to its apex 0.5 below the top, so it
never meets the opening, the cavity cannot reach it, and no rim is made: the top stays as it was,
and the cavity is the closed cone bounded by the moved side and the moved base, from the disk of
radius 0.6 at z = 2.7 to the point (0, 0, 3.5), its side 1 long. The result is the frustum less that
cone, one solid bounded by two shells. A kernel that cannot tell that the opening is never reached
may refuse; one that removes the top anyway, or runs the cavity up through it, is wrong.

**As a set:**

$$\{\,\rho \le \tfrac34(8 - z),\ 0 \le z \le 4\,\} \setminus \{\,\rho \le \tfrac34(3.5 - z),\ z \ge
2.7\,\}$$
