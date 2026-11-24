+++
title = "A plate with every edge rounded to half its thickness"
tags = ["fillet", "corner", "vanishing-face", "degenerate"]

[volume]
value = 6649.008700703104
exact = "2r·34·24 + (πr²/2)·2(34 + 24) + 4πr³/3 = 4896 + 558π, r = 3: the 34 × 24 rectangle at mid-height grown by r"
tex = '2r\cdot 34\cdot 24 + \tfrac{\pi r^2}{2}\cdot 2(34 + 24) + \tfrac{4}{3}\pi r^3 = 4896 + 558\pi,\quad r = 3'

[area]
value = 2838.3715789784806
exact = "2·34·24 + πr·2(34 + 24) + 4πr² = 1632 + 384π, r = 3"
tex = '2\cdot 34\cdot 24 + \pi r\cdot 2(34 + 24) + 4\pi r^2 = 1632 + 384\pi,\quad r = 3'
+++

A plate from the origin to (40, 30, 6). Round all twelve of its edges to radius 3, half its
thickness, in one operation. The result is the union of every ball of radius 3 inside the plate:
the plate made 3 smaller on every side, which leaves the rectangle 34 × 24 at z = 3 from (3, 3, 3)
to (37, 27, 3), then swelled by 3.

So the four side faces vanish: along each side the top round and the bottom round are the two
halves of one half-cylinder of radius 3 on the line at z = 3, three in from the side, and they
meet tangentially along the side's middle, z = 3. At each corner the three rounds of the top
vertex and the three of the bottom vertex all roll the same ball, centred three in from both sides
at z = 3, so the four upright edges keep no round at all, and each corner of the plate is a quarter
of that ball. The top and bottom faces are each the 34 × 24 rectangle.

**As a set:**

$$\bigl([0, 40]\times[0, 30]\times[0, 6] \ominus B_3\bigr) \oplus B_3$$
