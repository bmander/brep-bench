+++
title = "A box shelled to a wall 0.001 thick"
tags = ["shell", "inward", "planar", "sliver", "thin"]

[volume]
value = 0.29594000399998777
exact = "abc − (a − 2t)(b − 2t)(c − t) = 480 − 9.998·7.998·5.999, a = 10, b = 8, c = 6, t = 0.001"
tex = 'abc - (a - 2t)(b - 2t)(c - t) = 480 - 9.998\cdot 7.998\cdot 5.999,\quad t = 0.001'

[area]
value = 591.9160079999999
exact = "ab + 2(a + b)c + ab + 2(9.998 + 7.998)·5.999"
tex = 'ab + 2(a + b)c + ab + 2(9.998 + 7.998)\cdot 5.999'
+++

The box from the origin to (10, 8, 6). Shell it inward to a wall 0.001 thick, open at its top face,
z = 6. The outside of the wall is the box's other five faces, unmoved, and the inside is each of
them moved 0.001 inward along its normal, onto the surface offset 0.001 from it. The moved faces are
extended or trimmed where they meet one another, and run on until they meet the surface of an
opening, which is left as a rim between the outside and the inside.

The result is the box less the box from (0.001, 0.001, 0.001) to (9.999, 7.999, 6): a wall 10 000
times thinner than the box is wide, whose rim is a frame 0.001 wide. Its volume is small beside its
area, so a kernel whose tolerances are fixed lengths rather than fractions of the solid's size loses
it.
