+++
title = "The Bézier dome drilled through off centre"
tags = ["nurbs", "bezier", "boolean", "cut", "cylinder"]
[volume]
value = 2872.944892550934
exact = "3150 − ∬ over the hole's disc of the patch's height = 3150 − 88.18937971875π: the height is a bicubic polynomial in x and y, and over a disc of radius a about c it integrates to πa²h + (πa⁴/8)(h_xx + h_yy) + (πa⁶/96)h_xxyy at c"

[area]
value = 1431.7956857724895
exact = "no closed form (the domed top's area element is the root of a polynomial): OCCT and cad-kernel, built independently from this description, agree to 2e-11 (1431.7956857841493 and 1431.7956857608297; the mean)"
+++

The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with a round hole drilled
straight down through it.

The block stands on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20, its floor flat at z = 0, its four sides
upright, and its top the bicubic Bézier patch whose sixteen control points are
Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ) for i, j = 0 … 3, at these heights (row i, x = 20i/3; column j,
y = 20j/3):

| | j = 0 | j = 1 | j = 2 | j = 3 |
|---|---|---|---|---|
| i = 0 | 4 | 6 | 7 | 5 |
| i = 1 | 6 | 14 | 12 | 7 |
| i = 2 | 5 | 13 | 16 | 8 |
| i = 3 | 3 | 6 | 8 | 6 |

(S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ with the cubic Bernstein polynomials; x = 20u and y = 20v, so
the patch is the graph of a height over the square.)

The hole is the vertical cylinder of radius 3 about the line x = 12, y = 8: it runs from the
floor right up through the dome, so it opens in the floor and in the domed top, where its rim is
a space curve, and stays clear of the four sides. If you drill it with a cylinder, make the
cylinder run past the floor and past the top.
