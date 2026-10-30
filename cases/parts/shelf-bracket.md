+++
title = "A shelf bracket with a rounded inside corner"
tags = ["part", "extrude", "analytic", "fillet"]
origin = "geomsolver rust/examples/rounded_bracket.sv"

[volume]
value = 40137.3451754256
exact = "10·(120·20 + 80·20 + 8²(1 − π/4)) = 40640 − 160π"

[area]
value = 12393.13274122872
exact = "2(4064 − 16π) + 10·(20 + 92 + 4π + 72 + 20 + 100 + 120) = 12368 + 8π"
+++

An L-shaped plate bracket, 10 thick: its outline lies in the xz plane and is extruded from
y = −5 to y = 5. In the xz plane (the first number is x and the second z), the outline runs:

- from (0, 0) to (20, 0), the foot of the post;
- from (20, 0) up to (20, 92), the inside of the post;
- along a quarter circle of radius 8 centred at (28, 92), from (20, 92) to (28, 100), bulging
  toward the corner (20, 100) (its middle is at (28 − 4√2, 92 + 4√2));
- from (28, 100) to (100, 100), the underside of the arm;
- from (100, 100) up to (100, 120), the end of the arm;
- from (100, 120) back to (0, 120), the top;
- from (0, 120) down to (0, 0), the back of the post.

So it is a post 20 wide and 120 tall with an arm 20 deep reaching out to x = 100 along its top,
the inside corner between them rounded to radius 8, the round tangent to the post at z = 92 and
to the arm at x = 28.
