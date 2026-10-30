+++
title = "The Bézier dome sliced by a plane through a corner"
tags = ["nurbs", "bezier", "boolean", "cut", "plane", "vertex-on-plane"]

[volume]
value = 2854.4122952001485
exact = "no closed form (the cut line is where a bicubic height meets a plane, a curve of degree 6 in plan): OCCT and cad-kernel, built independently from this description, agree to 3e-09 (2854.412291226509 and 2854.412299173788; the mean)"

[area]
value = 1276.254114612174
exact = "no closed form: OCCT and cad-kernel, built independently from this description, agree to 2e-09 (1276.2541133495472 and 1276.2541158748008; the mean)"
+++

The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with everything above the
inclined plane z = 4 + x/2 cut away.

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

The solid is the part of that block on or below the plane z = 4 + x/2, which rises along x and is
level along y. The plane passes exactly through the block's corner (0, 0, 4), where the dome
meets the sides x = 0 and y = 0: the dome's height there is 4, its control point P₀₀. From that
corner the line where the plane meets the dome runs out across the top, leaving the corner along
(3, 2) in plan, to the side y = 20, which it reaches near x = 4.23. On the side x = 0 the plane
cuts the side down to the rectangle 0 ≤ z ≤ 4, touching the dome's edge there only at the
corner. Beyond the line, towards larger x, the dome lies below the plane and is kept; between
the line and the side x = 0 the top is the flat cut.
