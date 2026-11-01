+++
title = "A box shelled to half its width: the walls meet and nothing is hollowed"
tags = ["shell", "inward", "planar", "coincident", "degenerate"]

[volume]
value = 480
exact = "abc: the cavity has no volume, a = 10, b = 8, c = 6, t = b/2 = 4"
tex = 'abc,\quad a = 10,\ b = 8,\ c = 6,\ t = \tfrac{b}{2} = 4'

[area]
value = 376
exact = "2(ab + bc + ca): the box's own faces"
tex = '2(ab + bc + ca)'
+++

The box from the origin to (10, 8, 6). Shell it inward to a wall 4 thick, open at its top face, z =
6. The outside of the wall is the box's other five faces, unmoved, and the inside is each of them
moved 4 inward along its normal, onto the surface offset 4 from it. The moved faces are extended or
trimmed where they meet one another, and run on until they meet the surface of an opening, which is
left as a rim between the outside and the inside.

The thickness is half the box's width in y, so the walls at y = 0 and y = 8, moved 4 inward, meet in
the one plane y = 4: the cavity, the box from (4, 4, 4) to (6, 4, 6), is flat and has no volume, and
every point of the box is within 4 of a wall. Nothing is hollowed, and the result is the box itself,
its six faces as they were. A kernel that leaves the two coincident inside walls as a slit of zero
thickness, or faces bounding nothing, does not make it.
