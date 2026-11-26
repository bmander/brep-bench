+++
title = "A box slid along the plane of its faces"
tags = ["swept", "solid-sweep", "slide", "polyhedron", "coplanar", "adversarial"]
origin = "adversarial: swept/box-slid-aslant slid level, (8, 5, 0) in place of (8, 5, 3)"

[volume]
value = 632.0
exact = "abc + |dx|bc + |dy|ca + |dz|ab = (ab + a|dy| + b|dx|)c, a, b, c = 10, 6, 4, d = (8, 5, 0)"
tex = 'abc + |d_x|\,bc + |d_y|\,ca + |d_z|\,ab,\quad (a, b, c) = (10, 6, 4),\ d = (8, 5, 0)'

[area]
value = 519.4718490564528
exact = "2(ab + bc + ca) + 2(a|e₁×d| + b|e₂×d| + c|e₃×d|) = 444 + 8√89"
tex = '2(ab + bc + ca) + 2\left(a\,|e_1 \times d| + b\,|e_2 \times d| + c\,|e_3 \times d|\right) = 444 + 8\sqrt{89}'
+++

The solid a box fills as it slides along a straight line without turning. The box starts as the box
from (0, 0, 0) to (10, 6, 4) and slides along the vector (8, 5, 0), level, ending as the box from
(8, 5, 0) to (18, 11, 4). Make the solid of everything the box passes through.

It is `swept/box-slid-aslant` with the slide in the plane of the box's top and bottom, so those two
faces slide within their own planes: the result is a prism 4 high, from z = 0 to z = 4, over the
hexagon the rectangle from (0, 0) to (10, 6) sweeps, through (0, 0), (10, 0), (18, 5), (18, 11),
(8, 11) and (0, 6). Its top and bottom are those hexagons, and its six sides are rectangles 4 high.

**As a set:**

$$\bigl([0,10]\times[0,6]\times[0,4]\bigr) \oplus \{\,s\,(8,5,0) : 0 \le s \le 1\,\}$$
