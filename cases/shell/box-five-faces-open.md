+++
title = "A box shelled open at every face but one"
tags = ["shell", "inward", "planar", "extreme"]

[volume]
value = 80
exact = "ab·t, a = 10, b = 8, t = 1"
tex = 'ab\,t,\quad a = 10,\ b = 8,\ t = 1'

[area]
value = 196
exact = "2ab + 2(a + b)t"
tex = '2ab + 2(a + b)\,t'
+++

The box from the origin to (10, 8, 6). Shell it inward to a wall 1 thick, open at five of its faces,
every face but its bottom, z = 0. The outside of the wall is the bottom alone, unmoved, and the
inside is the bottom moved 1 inward along its normal, onto the surface offset 1 from it. The moved
faces are extended or trimmed where they meet one another, and run on until they meet the surface of
an opening, which is left as a rim between the outside and the inside.

Only the bottom is left as a wall. Moved 1 inward it is the plane z = 1, which runs on to the planes
of the four sides, so the result is the slab from the origin to (10, 8, 1): six faces, its top the
bottom moved and its four sides the rims of the side openings, each 1 high. A kernel that refuses to
shell with every face but one open, or that keeps the slivers of the sides, does not make it.
