+++
title = "A block under a bicubic Bézier dome"
tags = ["nurbs", "bezier", "surface"]

[volume]
value = 3150.0
exact = "400·Σzᵢⱼ/16 = 25·126: x = 20u and y = 20v, and each cubic Bernstein polynomial integrates to ¼ over [0, 1]"

[area]
value = 1309.3229262031609
exact = "no closed form (the patch's area element is the root of a polynomial): OCCT and cad-kernel, built independently from this description, agree to 3e-13 (1309.322926202983 and 1309.3229262033387; the mean)"
+++

A block standing on the square 0 ≤ x ≤ 20, 0 ≤ y ≤ 20 of the xy plane, its floor flat at z = 0,
its four sides upright in the planes x = 0, x = 20, y = 0 and y = 20, and its top a bicubic
Bézier patch.

The patch's sixteen control points are Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ) for i, j = 0, 1, 2, 3, so that
they stand over an even 4 × 4 grid of the square (x = 0, 20/3, 40/3, 20 and the same for y), at
these heights zᵢⱼ (row i, x = 20i/3; column j, y = 20j/3):

| | j = 0 | j = 1 | j = 2 | j = 3 |
|---|---|---|---|---|
| i = 0 | 4 | 6 | 7 | 5 |
| i = 1 | 6 | 14 | 12 | 7 |
| i = 2 | 5 | 13 | 16 | 8 |
| i = 3 | 3 | 6 | 8 | 6 |

The patch is S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ for u, v from 0 to 1, with the cubic Bernstein
polynomials B₀(t) = (1 − t)³, B₁(t) = 3t(1 − t)², B₂(t) = 3t²(1 − t), B₃(t) = t³. Because the
control points' x and y lie on an even grid, x = 20u and y = 20v, so the patch is the graph of
a height over the square, everywhere at least 3 above the floor, and its four edges are the
upright sides' top edges: each side is the flat region between the floor and the cubic curve
the patch's edge draws in that side's plane.
