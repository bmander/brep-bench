+++
title = "A cone frustum shelled open at its top, so thick that nothing is hollowed"
tags = ["shell", "inward", "analytic", "cone", "apex", "degenerate"]

[volume]
value = 263.89378290154264
exact = "πH(R² + Rr + r²)/3 = 84π: the cavity is empty, R = 6, r = 3, H = 4, t = 3.5"
tex = '\tfrac{\pi H}{3}\left(R^2 + Rr + r^2\right) = 84\pi,\quad R = 6,\ r = 3,\ H = 4,\ t = 3.5'

[area]
value = 282.7433388230814
exact = "π(R² + r²) + π(R + r)s = 90π: the frustum's own faces, s = 5"
tex = '\pi\left(R^2 + r^2\right) + \pi\left(R + r\right)s = 90\pi,\quad s = 5'
+++

A solid cone frustum on the z axis, its base the disk of radius 6 at z = 0 and its top the disk of
radius 3 at z = 4, so its side is 5 long and leans in at the half-angle α with sin α = 3/5; the
side, run on, would close at its apex (0, 0, 8). Shell it inward to a wall 3.5 thick, open at its
top face, z = 4. The outside of the wall is its side and its base, unmoved, and the inside is each
of them moved 3.5 inward along its normal, onto the surface offset 3.5 from it. The moved faces are
extended or trimmed where they meet one another, and run on until they meet the surface of an
opening, which is left as a rim between the outside and the inside.

The side, moved 3.5 inward, is the cone with the same half-angle whose apex is 3.5/sin α = 35/6
lower, at (0, 0, 13/6); the base, moved 3.5 up, is the plane z = 3.5, above that apex. The space
inside the moved side and above the moved base is empty: every point of the frustum is within 3.5
of its side or its base. Nothing is hollowed, no rim is made, and the result is the frustum itself,
its three faces as they were. A kernel that cannot tell that the cavity is empty may refuse; one
that removes the top anyway, or leaves faces bounding nothing, is wrong.

**As a set:**

$$K \setminus \{\,\rho \le \tfrac34(\tfrac{13}{6} - z),\ z \ge 3.5\,\} = K, \quad K = \{\,\rho \le
\tfrac34(8 - z),\ 0 \le z \le 4\,\}$$
