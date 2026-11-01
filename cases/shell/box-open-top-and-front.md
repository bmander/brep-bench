+++
title = "A box shelled open at two faces that share an edge"
tags = ["shell", "inward", "planar", "adjacent-openings"]

[volume]
value = 200
exact = "abc − (a − 2t)(b − t)(c − t), a = 10, b = 8, c = 6, t = 1"
tex = 'abc - (a - 2t)(b - t)(c - t),\quad a = 10,\ b = 8,\ c = 6,\ t = 1'

[area]
value = 446
exact = "236 outside + 166 inside + 24 top rim + 20 front rim"
tex = '\underbrace{ab + ac + 2bc}_{236} + \underbrace{8\cdot 7 + 8\cdot 5 + 2\cdot 7\cdot 5}_{166} + \underbrace{ab - 8\cdot 7}_{24} + \underbrace{ac - 8\cdot 5}_{20}'
+++

The box from the origin to (10, 8, 6). Shell it inward to a wall 1 thick, open at two faces that
share an edge, its top, z = 6, and its front, y = 0. The outside of the wall is the box's other four
faces, unmoved, and the inside is each of them moved 1 inward along its normal, onto the surface
offset 1 from it. The moved faces are extended or trimmed where they meet one another, and run on
until they meet the surface of an opening, which is left as a rim between the outside and the
inside.

The cavity is the box from (1, 0, 1) to (9, 7, 6): it reaches both openings, and the edge they
shared, along y = 0, z = 6, is cut away between x = 1 and x = 9. What is left of it is two pieces 1
long at its ends, where the top rim and the front rim meet. The result is the box less that cavity:
a bottom, a back and two sides, each 1 thick.
