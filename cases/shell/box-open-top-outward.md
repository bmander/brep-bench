+++
title = "A box shelled outward with its top open, its edges rounded"
tags = ["shell", "outward", "planar", "round"]

[volume]
value = 345.2182849062401
exact = "2tc(a + b) + πt²c + abt + πt²(a + b)/2 + 2πt³/3 = 296 + 47π/3, a = 10, b = 8, c = 6, t = 1"
tex = '2tc(a + b) + \pi t^2 c + ab\,t + \tfrac{\pi t^2 (a + b)}{2} + \tfrac{2\pi t^3}{3} = 296 + \tfrac{47\pi}{3}'

[area]
value = 731.672557568463
exact = "(296 + 32π) outside + 296 inside + (36 + π) rim = 628 + 33π"
tex = '\underbrace{296 + 32\pi}_{\text{outside}} + \underbrace{296}_{\text{inside}} + \underbrace{36 + \pi}_{\text{rim}} = 628 + 33\pi'
+++

The box from the origin to (10, 8, 6), shelled outward to a wall 1 thick, open at its top face, z =
6. The box's faces other than the top become the inside of the wall, and the wall is every point
outside the box within 1 of those faces, up to the plane of the top, z = 6, which is the rim. So the
outside of the wall is each face moved 1 outward, joined at the box's convex edges by a quarter
cylinder of radius 1 about the edge, and at the bottom corners by an eighth of a ball of radius 1
about the corner.

The result is the box grown by 1 (every point within 1 of it) and cut off at z = 6, less the box
itself: a cup whose inside is the box, with sharp edges, and whose outside is rounded, 23 faces in
all.
