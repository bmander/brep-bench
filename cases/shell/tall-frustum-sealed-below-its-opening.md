+++
title = "A tall cone frustum shelled open at its narrow top, the cavity a cone sealed well below it"
tags = ["shell", "inward", "analytic", "cone", "apex", "void", "degenerate"]

[volume]
value = 439.822971502571
exact = "πH(R² + Rr + r²)/3 − πa²b/3 = 152π − 12π = 140π: R = 7, r = 1, H = 8; the cavity a cone a = 3 across, b = 4 high"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) - \tfrac{\pi a^2 b}{3} = 152\pi - 12\pi = 140\pi,\quad R = 7,\ r = 1,\ H = 8,\ a = 3,\ b = 4'

[area]
value = 483.80526865282815
exact = "π(R² + r²) + π(R + r)s + πa² + πac = 130π + 24π = 154π: s = 10 the frustum's side, c = 5 the cavity's"
tex = '\pi\left(R^2 + r^2\right) + \pi\left(R + r\right)s + \pi a^2 + \pi a c = 154\pi,\quad s = 10,\ c = 5'
+++

A solid cone frustum on the z axis, its base the disk of radius 7 at z = 0 and its top the disk of
radius 1 at z = 8, so its side is 10 long and leans in at the half-angle α with sin α = 3/5; the
side, run on, would close at its apex (0, 0, 28/3). Shell it inward to a wall 2 thick, open at its
top face, z = 8. The outside of the wall is its side and its base, unmoved, and the inside is each
of them moved 2 inward along its normal, onto the surface offset 2 from it. The moved faces are
extended or trimmed where they meet one another, and run on until they meet the surface of an
opening, which is left as a rim between the outside and the inside.

The side, moved 2 inward, is the cone with the same half-angle whose apex is 2/sin α = 10/3 lower,
at (0, 0, 6); the base, moved 2 up, is the plane z = 2, which that cone meets in the circle of
radius 3. The top is narrower than the wall is thick, so the moved side, that cone run on, closes to
its apex 2 below it: it never meets the opening, the cavity cannot reach it, and no rim is made. The
top stays as it was, and the cavity is the closed cone from the disk of radius 3 at z = 2 to the
point (0, 0, 6), its side 5 long. The result is the frustum less that cone, one solid bounded by two
shells. A kernel that cannot tell that the opening is never reached may refuse; one that removes the
top anyway, or runs the cavity up through it, is wrong.

**As a set:**

$$\{\,\rho \le \tfrac34(\tfrac{28}{3} - z),\ 0 \le z \le 8\,\} \setminus \{\,\rho \le \tfrac34(6 -
z),\ z \ge 2\,\}$$
